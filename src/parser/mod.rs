use std::str::CharIndices;
use std::iter::Peekable;

/**
 * UnitDesignation: 'A' | 'F' | 'Army' | 'Fleet'
 * ProvinceDesignation: [a-zA-Z. ]+
 * HoldOrder: UnitDesignation? ProvinceDesignation ('H' | 'Hold' | 'Holds')
 * MoveOrder: UnitDesignation? ProvinceDesignation ('M' | 'Move' | 'Moves' | '-') 
 *            ProvinceDesignation
 * HoldOrMoveOrder: HoldOrder | MoveOrder
 * SupportOrder: UnitDesignation? ProvinceDesignation ('S' | 'Support' | 'Supports')
 *               HoldOrMoveOrder
 * ConvoyOrder: UnitDesignation? ProvinceDesignation ('C' | 'Convoy' | 'Convoys') HoldOrMoveOrder
 * Order: HoldOrder | MoveOrder | SupportOrder | ConvoyOrder
 */

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Unit {
    Army,
    Fleet
}

fn parse_unit(order_str: &str) -> Result<(Unit, &str), &str> {
    if order_str.starts_with("army") {
        Ok((Unit::Army, order_str.split_at(4).1))
    } else if order_str.starts_with("fleet") {
        Ok((Unit::Fleet, order_str.split_at(5).1))
    } else if order_str.starts_with("a") {
        Ok((Unit::Army, order_str.split_at(1).1))
    } else if order_str.starts_with("f") {
        Ok((Unit::Fleet, order_str.split_at(1).1))
    } else {
        Err(order_str)
    }
}

fn skip_whitespace(order_str: &str) -> Result<&str, !> {
    Ok(order_str.trim_start())
}

fn require_whitespace(order_str: &str) -> Result<&str, &str> {
    if order_str.starts_with(char::is_whitespace) {
        Ok(order_str.trim_start())
    } else {
        Err(order_str)
    }
}

struct ParseProvince<'a> {
    order_str: &'a str,
    inner: Peekable<CharIndices<'a>>
}

impl <'a> Iterator for ParseProvince<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let ParseProvince { order_str, ref mut inner } = *self;

        if inner.peek() == None {
            return None;
        }

        while let Some((_, c)) = inner.next() {
            if ! c.is_whitespace() {
                break;
            }
        }

        while let Some((i, c)) = inner.next() {
            if c.is_whitespace() {
                unsafe {
                    return Some(order_str.split_at(i));
                }
            }
        }

        return Some((order_str, ""));
    }
}

fn parse_province(order_str: &str) -> ParseProvince {
    ParseProvince { order_str: order_str, inner: order_str.char_indices().peekable() }
}

fn parse_hold(order_str: &str) -> Result<((), &str), &str> {
    if order_str.starts_with("holds") {
        Ok(((), order_str.split_at(5).1))
    } else if order_str.starts_with("hold") {
        Ok(((), order_str.split_at(4).1))
    } else if order_str.starts_with("h") {
        Ok(((), order_str.split_at(1).1))
    } else {
        Err(order_str)
    }
}

fn parse_hold_order(order_str: &str) -> Result<((Option<Unit>, &str), &str), &str> {
    let mut rem_order_str = order_str;
    let mut unit: Option<Unit> = None;

    if let Ok((u, remainder)) = parse_unit(rem_order_str) {
        unit = Some(u);

        rem_order_str = match require_whitespace(remainder) {
            Ok(rem) => rem,
            Err(_) => return Err(order_str)
        };
    } else {
        rem_order_str = skip_whitespace(rem_order_str).unwrap();
    }

    for (province, remainder) in parse_province(rem_order_str) {
        let remainder = match require_whitespace(remainder) {
            Ok(rem) => rem,
            Err(_) => return Err(order_str)
        };

        if let Ok((_, rem)) = parse_hold(remainder) {
            return Ok(((unit, province), rem));
        }
    }

    Err(order_str)
}

fn expect_eol(order_str: &str) -> Result<(), &str> {
    if order_str.len() == 0 {
        Ok(())
    } else {
        Err(order_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_unit_test() {
        assert_eq!(parse_unit("army ruhr"), Ok((Unit::Army, " ruhr")));
    }

    #[test]
    fn skip_whitespace_test() {
        assert_eq!(skip_whitespace("army"), Ok("army"));
        assert_eq!(skip_whitespace(" army"), Ok("army"));
        assert_eq!(skip_whitespace("  army"), Ok("army"));
    }

    #[test]
    fn require_whitespace_test() {
        assert_eq!(require_whitespace("army"), Err("army"));
        assert_eq!(require_whitespace(" army"), Ok("army"));
        assert_eq!(require_whitespace("  army"), Ok("army"));
    }

    #[test]
    fn parse_province_test() {
        let mut iter = parse_province("Eastern Mediterranean Sea ");

        assert_eq!(iter.next(), Some(("Eastern", " Mediterranean Sea ")));
        assert_eq!(iter.next(), Some(("Eastern Mediterranean", " Sea ")));
        assert_eq!(iter.next(), Some(("Eastern Mediterranean Sea", " ")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn parse_hold_test() {
        assert_eq!(parse_hold("holds foo"), Ok(((), " foo")));
        assert_eq!(parse_hold("hold foo"), Ok(((), " foo")));
        assert_eq!(parse_hold("h foo"), Ok(((), " foo")));
        assert_eq!(parse_hold("foo"), Err("foo"));
    }

    #[test]
    fn parse_hold_order_test() {
        assert_eq!(parse_hold_order("a brest h"), Ok(((Some(Unit::Army), "brest"), "")));
        assert_eq!(parse_hold_order("fleet eastern med holds"), Ok(((Some(Unit::Fleet), "eastern med"), "")));
        assert_eq!(parse_hold_order("western med sea holds foo bar"), Ok(((None, "western med sea"), " foo bar")));
    }
}
