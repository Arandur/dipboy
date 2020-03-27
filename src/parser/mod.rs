mod combinators;
mod types;

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
pub enum Unit {
    Army,
    Fleet
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HoldOrder<'a> {
    unit: Option<Unit>,
    province: &'a str
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MoveOrder<'a> {
    unit: Option<Unit>,
    source: &'a str,
    dest: &'a str
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HoldOrMoveOrder<'a> {
    Hold(HoldOrder<'a>),
    Move(MoveOrder<'a>)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SupportOrder<'a> {
    unit: Option<Unit>,
    province: &'a str,
    order: HoldOrMoveOrder<'a>
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConvoyOrder<'a> {
    unit: Option<Unit>,
    province: &'a str,
    order: MoveOrder<'a>
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Order<'a> {
    Hold(HoldOrder<'a>),
    Move(MoveOrder<'a>),
    Support(SupportOrder<'a>),
    Convoy(ConvoyOrder<'a>)
}

pub fn parse_unit(order_str: &str) -> Result<(Unit, &str), &str> {
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

pub fn optional<T>(parser: impl Fn(&str) -> Result<(T, &str), &str>) -> impl Fn(&str) -> Result<(Option<T>, &str), !> {
    move |s| match parser(s) {
        Ok((val, rem)) => Ok((Some(val), rem)),
        Err(rem) => Ok((None, rem))
    }
}

pub fn skip_whitespace(order_str: &str) -> Result<&str, !> {
    Ok(order_str.trim_start())
}

pub fn require_whitespace(order_str: &str) -> Result<&str, &str> {
    if order_str.starts_with(char::is_whitespace) {
        Ok(order_str.trim_start())
    } else {
        Err(order_str)
    }
}

pub struct ParseProvince<'a> {
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
            if c.is_alphabetic() || c == '.' || c == '(' || c == ')' {
                break;
            }
        }

        while let Some((i, c)) = inner.next() {
            if !(c.is_alphabetic() || c == '.' || c == '(' || c == ')') {
                return Some(order_str.split_at(i));
            }
        }

        return Some((order_str, ""));
    }
}

pub fn parse_province(order_str: &str) -> ParseProvince {
    ParseProvince { order_str: order_str, inner: order_str.char_indices().peekable() }
}

pub fn parse_hold(order_str: &str) -> Result<((), &str), &str> {
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

pub fn expect_eol(order_str: &str) -> Result<(), &str> {
    if order_str.len() == 0 {
        Ok(())
    } else {
        Err(order_str)
    }
}

pub fn parse_hold_order(order_str: &str) -> Result<((Option<Unit>, &str), &str), &str> {
    let (unit, mut rem_order_str) = optional(parse_unit)(order_str).unwrap();

    rem_order_str = skip_whitespace(rem_order_str).unwrap();

    for (province, remainder) in parse_province(rem_order_str) {
        let remainder = match require_whitespace(remainder) {
            Ok(rem) => rem,
            Err(_) => return Err(order_str)
        };

        if let Ok((_, rem)) = parse_hold(remainder) {
            if let Ok(_) = expect_eol(rem) {
                return Ok(((unit, province), rem));
            }
        }
    }

    Err(order_str)
}

pub fn parse_to(order_str: &str) -> Result<((), &str), &str> {
    if order_str.starts_with("-") {
        Ok(((), order_str.split_at(1).1))
    } else if order_str.starts_with("to") {
        Ok(((), order_str.split_at(2).1))
    } else {
        Err(order_str)
    }
}

pub fn parse_move_order(order_str: &str) -> Result<((Option<Unit>, &str, &str), &str), &str> {
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

    for (source, remainder) in parse_province(rem_order_str) {
        let remainder = skip_whitespace(remainder).unwrap();

        if let Ok((_, rem)) = parse_to(remainder) {
            rem_order_str = skip_whitespace(rem).unwrap();

            for (dest, remainder) in parse_province(rem_order_str) {
                if let Ok(_) = expect_eol(remainder) {
                    return Ok(((unit, source, dest), ""));
                }
            }
        }
    }

    Err(order_str)
}

pub fn parse_support(order_str: &str) -> Result<((), &str), &str> {
    if order_str.starts_with("supports") {
        Ok(((), order_str.split_at(8).1))
    } else if order_str.starts_with("support") {
        Ok(((), order_str.split_at(7).1))
    } else if order_str.starts_with("s") {
        Ok(((), order_str.split_at(1).1))
    } else {
        Err(order_str)
    }
}

pub fn parse_support_order(order_str: &str) -> Result<((Option<Unit>, &str, Option<Unit>, &str, &str), &str), &str> {
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

    for (prov, remainder) in parse_province(rem_order_str) {
        let remainder = skip_whitespace(remainder).unwrap();

        if let Ok((_, rem)) = parse_support(remainder) {
            rem_order_str = skip_whitespace(rem).unwrap();

            if let Ok(((unit2, source, dest), rem)) = parse_move_order(rem_order_str) {
                return Ok(((unit, prov, unit2, source, dest), rem));
            } else if let Ok(((unit2, source), rem)) = parse_hold_order(rem_order_str) {
                return Ok(((unit, prov, unit2, source, source), rem));
            }
        }
    }

    Err(order_str)
}

pub fn parse_convoy(order_str: &str) -> Result<((), &str), &str> {
    if order_str.starts_with("convoys") {
        Ok(((), order_str.split_at(7).1))
    } else if order_str.starts_with("convoy") {
        Ok(((), order_str.split_at(6).1))
    } else if order_str.starts_with("c") {
        Ok(((), order_str.split_at(1).1))
    } else {
        Err(order_str)
    }
}

pub fn parse_convoy_order(order_str: &str) -> Result<((Option<Unit>, &str, Option<Unit>, &str, &str), &str), &str> {
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

    for (prov, remainder) in parse_province(rem_order_str) {
        let remainder = skip_whitespace(remainder).unwrap();

        if let Ok((_, rem)) = parse_convoy(remainder) {
            rem_order_str = skip_whitespace(rem).unwrap();

            if let Ok(((unit2, source, dest), rem)) = parse_move_order(rem_order_str) {
                return Ok(((unit, prov, unit2, source, dest), rem));
            }
        }
    }

    Err(order_str)
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
        assert_eq!(parse_hold_order("western med sea holds"), Ok(((None, "western med sea"), "")));
    }

    #[test]
    fn parse_move_order_test() {
        assert_eq!(parse_move_order("a brest - paris"), Ok(((Some(Unit::Army), "brest", "paris"), "")));
        assert_eq!(parse_move_order("a brest-paris"), Ok(((Some(Unit::Army), "brest", "paris"), "")));
        assert_eq!(parse_move_order("paris-burgundy"), Ok(((None, "paris", "burgundy"), "")));
        assert_eq!(parse_move_order("f western med to spain (sc)"), Ok(((Some(Unit::Fleet), "western med", "spain (sc)"), "")));
    }

    #[test]
    fn parse_support_order_test() {
        assert_eq!(parse_support_order("a brest s a paris h"), Ok(((Some(Unit::Army), "brest", Some(Unit::Army), "paris", "paris"), "")));
        assert_eq!(parse_support_order("a brest s paris-burgundy"), Ok(((Some(Unit::Army), "brest", None, "paris", "burgundy"), "")));
    }

    #[test]
    fn parse_convoy_order_test() {
        assert_eq!(parse_convoy_order("f brest convoys paris-burgundy"), Ok(((Some(Unit::Fleet), "brest", None, "paris", "burgundy"), "")));
    }
}
