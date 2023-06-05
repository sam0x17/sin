use crate::{TokenStream2, TokenTree};

pub trait ParseState: Sized + Clone + PartialEq {
    fn new() -> Self;
}

impl ParseState for Stateless {
    fn new() -> Self {
        Stateless
    }
}

#[derive(Clone, Debug)]
pub struct StatefulParser<T: ParseState> {
    pub state: T,
    vec: Vec<TokenTree>,
}

#[derive(Clone, PartialEq)]
pub struct Stateless;

pub type Parser = StatefulParser<Stateless>;

impl<T: ParseState> From<TokenStream2> for StatefulParser<T> {
    fn from(value: TokenStream2) -> Self {
        let mut vec = value.into_iter().collect::<Vec<TokenTree>>();
        vec.reverse();
        StatefulParser {
            state: T::new(),
            vec,
        }
    }
}

impl<T: ParseState> StatefulParser<T> {
    pub fn fork(&self) -> Self {
        (*self).clone()
    }

    pub fn has_next(&self) -> bool {
        !self.vec.is_empty()
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        self.vec.pop()
    }

    pub fn peek(&self, _tt: TokenTree) -> bool {
        let Some(next_tt) = self.vec.last() else { return false };
        matches!(next_tt, _tt)
    }
}
