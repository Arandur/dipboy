use std::mem;
use std::marker::PhantomData;

use super::types;

pub trait Parser<'a> : Clone {
    type Item;
    type Result: Iterator<Item=(Self::Item, &'a str)> + Clone;

    fn parse(&self, s: &'a str) -> Self::Result;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Never;

impl <'a> Parser<'a> for Never {
    type Item = !;
    type Result = NeverParse<'a>;

    fn parse(&self, _: &str) -> NeverParse<'a> {
        NeverParse {
            data: PhantomData
        }
    }
}

pub fn never<'a>() -> impl Parser<'a, Item = !> {
    Never
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct NeverParse<'a> {
    data: PhantomData<&'a ()>
}

impl <'a> Iterator for NeverParse<'a> {
    type Item = (!, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Empty;

impl <'a> Parser<'a> for Empty {
    type Item = ();
    type Result = EmptyParse<'a>;

    fn parse(&self, s: &'a str) -> EmptyParse<'a> {
        EmptyParse {
            s: Some(s)
        }
    }
}

pub fn empty<'a>() -> impl Parser<'a, Item = ()> {
    Empty
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct EmptyParse<'a> {
    s: Option<&'a str>
}

impl <'a> Iterator for EmptyParse<'a> {
    type Item = ((), &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        mem::replace(&mut self.s, None).map(|s| ((), s))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Tag<'a> {
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

pub fn tag<'a>(value: &'a str) -> impl Parser<'a, Item = &'a str> {
    Tag { tag: value }
}

#[derive(Copy, Clone, Debug)]
struct TagParse<'a> {
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

#[derive(Clone)]
struct Optional<'a, T: Parser<'a>> {
    inner: T,
    data: PhantomData<&'a ()>
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

pub fn optional<'a, T: Parser<'a>>(p: T) -> impl Parser<'a, Item = Option<<T as Parser<'a>>::Item>> {
    Optional {
        inner: p,
        data: PhantomData
    }
}

#[derive(Clone)]
struct OptionalParse<'a, T: Parser<'a>> {
    s: &'a str,
    inner: Option<<T as Parser<'a>>::Result>
}

impl <'a, T: Parser<'a>> Iterator for OptionalParse<'a, T> {
    type Item = (Option<<T as Parser<'a>>::Item>, &'a str);

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

#[derive(Clone)]
struct Either<'a, T: Parser<'a>, U: Parser<'a>> {
    left: T,
    right: U,
    data: PhantomData<&'a ()>
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

pub fn either<'a, T: Parser<'a>, U: Parser<'a>>(left: T, right: U) -> impl Parser<'a, Item = types::Either<<T as Parser<'a>>::Item, <U as Parser<'a>>::Item>> {
    Either {
        left: left,
        right: right,
        data: PhantomData
    }
}

#[derive(Clone)]
struct EitherParse<'a, T: Parser<'a>, U: Parser<'a>> {
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

#[derive(Clone)]
struct Chain<'a, T: Parser<'a>, U: Parser<'a>> 
    where <T as Parser<'a>>::Item: Clone + Sized {
   first: T,
   second: U,
   data: PhantomData<&'a ()>
}

impl <'a, T: Parser<'a>, U: Parser<'a>> Parser<'a> for Chain<'a, T, U> 
    where <T as Parser<'a>>::Item: Clone + Sized {
    type Item = (<T as Parser<'a>>::Item, <U as Parser<'a>>::Item);
    type Result = ChainParse<'a, T, U>;

    fn parse(&self, s: &'a str) -> Self::Result {
        ChainParse {
            first_it: self.first.parse(s),
            second: self.second.clone(),
            inner: None
        }
    }
}

pub fn chain<'a, T: Parser<'a>, U: Parser<'a>>(first: T, second: U) -> impl Parser<'a, Item = (<T as Parser<'a>>::Item, <U as Parser<'a>>::Item)>
    where <T as Parser<'a>>::Item: Clone + Sized {
    Chain {
        first: first,
        second: second,
        data: PhantomData
    }
}

#[derive(Clone)]
struct ChainParse<'a, T: Parser<'a>, U: Parser<'a>> 
    where <T as Parser<'a>>::Item: Clone + Sized {
    first_it: <T as Parser<'a>>::Result,
    second: U,
    inner: Option<(<T as Parser<'a>>::Item, <U as Parser<'a>>::Result)>
}

impl <'a, T: Parser<'a>, U: Parser<'a>> Iterator for ChainParse<'a, T, U> 
    where <T as Parser<'a>>::Item: Clone + Sized {
    type Item = ((<T as Parser<'a>>::Item, <U as Parser<'a>>::Item), &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            Some((first_result, second_it)) => {
                match second_it.next() {
                    Some((second_result, remainder)) => Some(((first_result.clone(), second_result), remainder)),
                    None => {
                        mem::replace(&mut self.inner, None);
                        Self::next(self)
                    }
                }
            },
            None => {
                match self.first_it.next() {
                    Some((first_result, remainder)) => {
                        mem::replace(&mut self.inner, Some((first_result, self.second.parse(remainder))));
                        Self::next(self)
                    },
                    None => None
                }
            }
        }
    }
}

struct Map<'a, T, R, F> 
    where T: Parser<'a>,
          F: Fn(<T as Parser<'a>>::Item) -> R + Clone {
    inner: T,
    func: F,
    data: PhantomData<&'a R>
}

impl <'a, T, R, F> Clone for Map<'a, T, R, F>
    where T: Parser<'a>,
          F: Fn(<T as Parser<'a>>::Item) -> R + Clone {
    fn clone(&self) -> Self {
        Map {
            inner: self.inner.clone(),
            func: self.func.clone(),
            data: PhantomData
        }
    }
}

impl <'a, T, R, F> Parser<'a> for Map<'a, T, R, F> 
    where T: Parser<'a>,
          F: Fn(<T as Parser<'a>>::Item) -> R + Clone {
    type Item = R;
    type Result = MapParse<'a, T, R, F>;

    fn parse(&self, s: &'a str) -> Self::Result {
        MapParse {
            inner: self.inner.parse(s),
            func: self.func.clone(),
            data: PhantomData
        }
    }
}

fn map<'a, T, R, F>(inner: T, func: F) -> impl Parser<'a, Item = R>
    where T: Parser<'a>,
          F: Fn(<T as Parser<'a>>::Item) -> R + Clone,
          R: 'a {
    Map {
        inner: inner,
        func: func,
        data: PhantomData
    }
}

struct MapParse<'a, T, R, F> 
    where T: Parser<'a>,
          F: Fn(<T as Parser<'a>>::Item) -> R + Clone {
    inner: <T as Parser<'a>>::Result,
    func: F,
    data: PhantomData<&'a R>
}

impl <'a, T, R, F> Clone for MapParse<'a, T, R, F>
    where T: Parser<'a>,
          F: Fn(<T as Parser<'a>>::Item) -> R + Clone {
    fn clone(&self) -> Self {
        MapParse {
            inner: self.inner.clone(),
            func: self.func.clone(),
            data: PhantomData
        }
    }
}

impl <'a, T, R, F> Iterator for MapParse<'a, T, R, F> 
    where T: Parser<'a>,
          F: Fn(<T as Parser<'a>>::Item) -> R + Clone {
    type Item = (R, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(res, rem)| ((self.func)(res), rem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_test() {
        let parser = never();
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), None);
    }

    #[test]
    fn tag_test() {
        let parser = tag("abc");
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), Some(("abc", "de")));
        assert_eq!(results.next(), None);

        let parser = tag("abc");
        let mut results = parser.parse("def");

        assert_eq!(results.next(), None);
    }

    #[test]
    fn optional_test() {
        let parser = optional(tag("abc"));
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), Some((Some("abc"), "de")));
        assert_eq!(results.next(), Some((None, "abcde")));
        assert_eq!(results.next(), None);

        let parser = optional(tag("abc"));
        let mut results = parser.parse("def");

        assert_eq!(results.next(), Some((None, "def")));
        assert_eq!(results.next(), None);
    }

    #[test]
    fn either_test() {
        let parser = either(tag("a"), tag("b"));
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), Some((types::Either::Left("a"), "bcde")));
        assert_eq!(results.next(), None);

        let mut results = parser.parse("bcde");

        assert_eq!(results.next(), Some((types::Either::Right("b"), "cde")));
        assert_eq!(results.next(), None);
    }

    #[test]
    fn chain_test() {
        let parser = chain(tag("a"), tag("b"));
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), Some((("a", "b"), "cde")));
        assert_eq!(results.next(), None);

        let parser = chain(optional(tag("a")), optional(tag("a")));
        let mut results = parser.parse("aab");

        assert_eq!(results.next(), Some(((Some("a"), Some("a")), "b")));
        assert_eq!(results.next(), Some(((Some("a"), None), "ab")));
        assert_eq!(results.next(), Some(((None, Some("a")), "ab")));
        assert_eq!(results.next(), Some(((None, None), "aab")));
        assert_eq!(results.next(), None);
    }

    #[test]
    fn map_test() {
        let parser = map(tag("a"), str::len);
        let mut results = parser.parse("abcde");

        assert_eq!(results.next(), Some((1, "bcde")));
        assert_eq!(results.next(), None);
    }
}
