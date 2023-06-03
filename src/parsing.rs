use crate::{TokenStream2, TokenTree};

pub trait ParseState: Sized + Clone + PartialEq {
    fn new() -> Self;
}
#[derive(Clone, PartialEq)]
pub struct Stateless;

impl ParseState for Stateless {
    fn new() -> Self {
        Stateless
    }
}

#[derive(Clone, Debug)]
pub struct Parser<T: ParseState> {
    pub state: T,
    vec: Vec<TokenTree>,
}

impl<T: ParseState> From<TokenStream2> for Parser<T> {
    fn from(value: TokenStream2) -> Self {
        let mut vec = value.into_iter().collect::<Vec<TokenTree>>();
        vec.reverse();
        Parser {
            state: T::new(),
            vec,
        }
    }
}

impl<T: ParseState> Parser<T> {
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
