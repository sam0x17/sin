use crate::{
    token_stream::{Peekable, TSIterator},
    *,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseError {
    pub messages: Vec<ErrorMessage>,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for message in &self.messages {
            message.fmt(f)?;
        }
        Ok(())
    }
}

impl core::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for message in &self.messages {
            message.fmt(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErrorMessage {
    pub span: Span,
    pub message: InStr,
}

impl core::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("error: {}", self.message))?;
        Ok(())
    }
}

impl core::fmt::Debug for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("error: {}", self.message))?;
        Ok(())
    }
}

impl Spanned for ErrorMessage {
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> PartialEq<&'a str> for ErrorMessage {
    fn eq(&self, other: &&str) -> bool {
        self.message == *other
    }
}

impl PartialEq<str> for ErrorMessage {
    fn eq(&self, other: &str) -> bool {
        self.message == other
    }
}

impl ParseError {
    pub fn new() -> ParseError {
        ParseError {
            messages: Vec::new(),
        }
    }

    pub fn expected_token(&self, expected: TokenPattern, found: Option<Token>, span: Span) -> Self {
        let mut this = self.clone();
        let message = match found {
            Some(found) => format!("expected {expected}, found `{found}`").into(),
            None => format!("expected {expected}, found end of tokens").into(),
        };
        this.messages.push(ErrorMessage { span, message });
        this
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Clone)]
pub struct Parser<'a, T: Default + Clone = ()>(TSIterator<'a, T>, Span);

impl<'a, T: Default + Clone> Iterator for Parser<'a, T> {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'a, T: Default + Clone> Peekable<TokenTree> for Parser<'a, T> {
    fn peek_n(&self, n: isize) -> Option<TokenTree> {
        self.0.peek_n(n)
    }

    fn peek(&self) -> Option<TokenTree> {
        self.0.peek()
    }
}

impl<'a, T: Default + Clone> Parser<'a, T> {
    pub fn new(iter: TSIterator<'a, T>, span: Span) -> Self {
        Parser(iter, span)
    }

    pub fn state_mut(&mut self) -> &mut T {
        &mut self.0.state
    }

    pub fn state(&self) -> &T {
        &self.0.state
    }

    pub fn span(&self) -> Span {
        self.1
    }

    pub fn set_span(&mut self, span: Span) {
        self.1 = span
    }

    pub fn parse<I: Parse>(&mut self) -> ParseResult<I> {
        I::parse(self)
    }
}

impl<'a, T: Default + Clone> From<TSIterator<'a, T>> for Parser<'a, T> {
    fn from(value: TSIterator<'a, T>) -> Self {
        Parser(value, Span::call_site())
    }
}

impl<'a, T: Default + Clone> From<&'a TokenStream> for Parser<'a, T> {
    fn from(value: &'a TokenStream) -> Self {
        value.to_state_parser()
    }
}

pub trait Parse:
    Sized + Clone + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash + core::fmt::Debug + ToTokens
{
    fn parse<'a, T: Default + Clone>(input: &mut Parser<'a, T>) -> ParseResult<Self>;

    fn parse_tokens(tokens: impl Into<TokenStream>) -> ParseResult<Self> {
        let tokens = tokens.into();
        let mut input: Parser = tokens.to_parser();
        let parsed = input.parse::<Self>()?;
        input.parse::<Nothing>()?;
        Ok(parsed)
    }
}

impl<P: Parse> From<P> for TokenStream {
    fn from(value: P) -> Self {
        value.to_token_stream()
    }
}

pub trait ToTokens: Sized + Clone + core::fmt::Debug {
    fn to_token_stream(&self) -> TokenStream;
}

pub fn parse<T: Parse>(tokens: impl Into<TokenStream>) -> ParseResult<T> {
    T::parse_tokens(tokens)
}
