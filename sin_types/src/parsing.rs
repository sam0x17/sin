use crate::{token_stream::TSIterator, *};

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

pub type ParseResult<T> = Result<T, ParseError>;

// pub struct Parser<'a>(TSIterator<'a>);

// impl<'a> From<TSIterator<'a>> for Parser<'a> {
//     fn from(value: TSIterator<'a>) -> Self {
//         Parser(value)
//     }
// }

// impl<'a> Parser<'a> {

// }

pub trait Parse: Sized {
    fn parse<'a>(input: impl Into<TSIterator<'a>>) -> ParseResult<Self>;
}

pub struct Ident {
    token_tree: TokenTree,
}

// impl Parse for Ident {
//     fn parse<'a>(input: impl Into<TSIterator<'a>>) -> ParseResult<Self> {
//         let mut input: TSIterator = input.into();
//         let Some(token_tree) = input.next() else { ParseError }
//         Ok(Ident {
//             token_tree: t![#hey],
//         })
//     }
// }
