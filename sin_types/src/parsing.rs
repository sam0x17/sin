use crate::{token_stream::TSIterator, *};
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ParseError {
    messages: Vec<ErrorMessage>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct ErrorMessage {
    start_span: Span,
    end_span: Span,
    message: InStr,
}

impl ParseError {
    pub fn new() -> ParseError {
        ParseError {
            messages: Vec::new(),
        }
    }

    pub fn missing_token(&self, expected_token: Token, span: Span) -> Self {
        let mut this = self.clone();
        this.messages.push(ErrorMessage {
            start_span: span,
            end_span: span,
            message: format!("expected `{expected_token}`").into(),
        });
        this
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser<'a, T = ()>(TSIterator<'a, T>, Span);

impl<'a, T> Parser<'a, T> {
    pub fn state_mut(&mut self) -> &mut T {
        &mut self.0.state
    }

    pub fn state(&self) -> &T {
        &self.0.state
    }

    pub fn current_span(&self) -> Span {
        self.1
    }

    pub fn set_span(&mut self, span: Span) {
        self.1 = span
    }
}

pub trait Parse: Sized {
    fn parse<'a>(input: impl Into<TSIterator<'a>>) -> ParseResult<Self>;
}

pub struct Ident {
    token_tree: TokenTree,
}

// impl Parse for Ident {
//     fn parse<'a>(input: impl Into<TSIterator<'a>>) -> ParseResult<Self> {
//         let mut input: TSIterator = input.into();
//         let Some(token_tree) = input.next() else {
//             return Err(ParseError::new().missing_token(t![#*], token_tree.span()));
//         };
//         Ok(Ident {
//             token_tree: t![#hey],
//         })
//     }
// }
