use std::mem;
use std::marker::PhantomData;

use super::types;

pub trait Parser<'a> {
    type Item;
    type Result: Iterator<Item=(Self::Item, &'a str)>;

    fn parse(&self, s: &'a str) -> Self::Result;
}

pub struct Tag<'a> {
    tag: &'a str
}

impl <'a> Parser<'a> for Tag<'a> {
    type Item = &'a str;
    type Result = TagParse<'a>;

    fn parse(&self, s: &'a str) -> TagParse<'a> {
        TagParse {
            s: s,
            tag: Some(self.tag)
        }
    }
}

pub fn tag<'a>(value: &'a str) -> Tag<'a> {
    Tag { tag: value }
}

pub struct TagParse<'a> {
    s: &'a str,
    tag: Option<&'a str>
}

impl <'a> Iterator for TagParse<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let old_tag = mem::replace(&mut self.tag, None);

        old_tag.and_then(|tag| if self.s.starts_with(tag) {
            Some((tag, &self.s[tag.len()..]))
        } else {
            None
        })
    }
}

pub struct Optional<'a, T: Parser<'a>> {
    inner: T,
    data: PhantomData<&'a T>
}

impl <'a, T: Parser<'a>> Parser<'a> for Optional<'a, T> {
    type Item = Option<<T as Parser<'a>>::Item>;
    type Result = OptionalParse<'a, T>;

    fn parse(&self, s: &'a str) -> Self::Result {
        OptionalParse {
            s: s,
            inner: Some(self.inner.parse(s))
        }
    }
}

pub fn optional<'a, T: Parser<'a>>(p: T) -> Optional<'a, T> {
    Optional {
        inner: p,
        data: PhantomData
    }
}

pub struct OptionalParse<'a, P: Parser<'a>> {
    s: &'a str,
    inner: Option<<P as Parser<'a>>::Result>
}

impl <'a, P: Parser<'a>> Iterator for OptionalParse<'a, P> {
    type Item = (Option<<P as Parser<'a>>::Item>, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            Some(inner) => match inner.next() {
                Some((value, remainder)) => Some((Some(value), remainder)),
                None => {
                    mem::replace(&mut self.inner, None);
                    Some((None, self.s))
                }
            },
            None => None
        }
    }
}

pub struct Either<'a, T: Parser<'a>, U: Parser<'a>> {
    left: T,
    right: U,
    data: PhantomData<&'a T>
}

impl <'a, T: Parser<'a>, U: Parser<'a>> Parser<'a> for Either<'a, T, U> {
    type Item = types::Either<<T as Parser<'a>>::Item, <U as Parser<'a>>::Item>;
    type Result = EitherParse<'a, T, U>;

    fn parse(&self, s: &'a str) -> Self::Result {
        EitherParse {
            left: Some(self.left.parse(s)),
            right: Some(self.right.parse(s))
        }
    }
}

pub fn either<'a, T: Parser<'a>, U: Parser<'a>>(left: T, right: U) -> Either<'a, T, U> {
    Either {
        left: left,
        right: right,
        data: PhantomData
    }
}

pub struct EitherParse<'a, T: Parser<'a>, U: Parser<'a>> {
    left: Option<<T as Parser<'a>>::Result>,
    right: Option<<U as Parser<'a>>::Result>
}

impl <'a, T: Parser<'a>, U: Parser<'a>> Iterator for EitherParse<'a, T, U> {
    type Item = (types::Either<<T as Parser<'a>>::Item, <U as Parser<'a>>::Item>, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        match (&mut self.left, &mut self.right) {
            (Some(left), _) => match left.next() {
                Some((value, remainder)) => Some((types::Either::Left(value), remainder)),
                None => {
                    mem::replace(&mut self.left, None);
                    Self::next(self)
                }
            },
            (None, Some(right)) => match right.next() {
                Some((value, remainder)) => Some((types::Either::Right(value), remainder)),
                None => {
                    mem::replace(&mut self.right, None);
                    None
                }
            },
            (None, None) => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_test() {
        let mut parser = tag("abc");
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), Some(("abc", "de")));
        assert_eq!(results.next(), None);

        let mut parser = tag("abc");
        let mut results = parser.parse("def");

        assert_eq!(results.next(), None);
    }

    #[test]
    fn optional_test() {
        let mut parser = optional(tag("abc"));
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), Some((Some("abc"), "de")));
        assert_eq!(results.next(), Some((None, "abcde")));
        assert_eq!(results.next(), None);

        let mut parser = optional(tag("abc"));
        let mut results = parser.parse("def");

        assert_eq!(results.next(), Some((None, "def")));
        assert_eq!(results.next(), None);
    }

    #[test]
    fn either_test() {
        let mut parser = either(tag("a"), tag("b"));
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), Some((types::Either::Left("a"), "bcde")));
        assert_eq!(results.next(), None);

        let mut results = parser.parse("bcde");

        assert_eq!(results.next(), Some((types::Either::Right("b"), "cde")));
        assert_eq!(results.next(), None);
    }
}
