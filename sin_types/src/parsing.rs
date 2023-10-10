use crate::{
    span::Spanned,
    token_stream::{Peekable, TSIterator},
    *,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ParseError {
    messages: Vec<ErrorMessage>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct ErrorMessage {
    span: Span,
    message: InStr,
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
}

pub trait ParseTokens: Parse {
    fn parse(tokens: impl Into<TokenStream>) -> ParseResult<Self> {
        let tokens = tokens.into();
        let mut input: Parser = tokens.to_parser();
        let parsed = input.parse::<Self>()?;
        input.parse::<Nothing>()?;
        Ok(parsed)
    }
}

pub trait ToTokens: Sized + Clone + core::fmt::Debug {
    fn to_token_stream(&self) -> TokenStream;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Nothing;

impl ToTokens for Nothing {
    fn to_token_stream(&self) -> TokenStream {
        TokenStream::new()
    }
}

impl Parse for Nothing {
    fn parse<'a, T: Default + Clone>(input: &mut Parser<'a, T>) -> ParseResult<Self> {
        let Some(token) = input.next() else {
            return Ok(Nothing {});
        };
        let span = token.span();
        return Err(ParseError::new().expected_token(pat![], Some(token.into()), span));
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Ident {
    span: Span,
    ident: InStr,
}

impl<'a, T: AsRef<str>> PartialEq<T> for Ident {
    fn eq(&self, other: &T) -> bool {
        self.ident == other.as_ref()
    }
}

impl From<Ident> for Token {
    fn from(value: Ident) -> Self {
        Token::Ident(value.ident)
    }
}

impl From<Ident> for TokenTree {
    fn from(value: Ident) -> Self {
        TokenTree::Leaf(Token::Ident(value.ident), value.span)
    }
}

impl Parse for Ident {
    fn parse<'a, T: Default + Clone>(input: &mut Parser<'a, T>) -> ParseResult<Self> {
        let Some(token_tree) = input.next() else {
            return Err(ParseError::new().expected_token(pat![!ident], None, input.span()));
        };
        let TokenTree::Leaf(token, span) = token_tree else {
            return Err(ParseError::new().expected_token(
                pat![!ident],
                Some(token_tree.into()),
                input.span(),
            ));
        };
        let Token::Ident(ident) = token else {
            return Err(ParseError::new().expected_token(pat![!ident], Some(token), input.span()));
        };
        Ok(Ident { ident, span })
    }
}

impl ToTokens for Ident {
    fn to_token_stream(&self) -> TokenStream {
        [TokenTree::Leaf((*self).into(), self.span)][..].into()
    }
}

#[test]
fn test_parse_nothing() {
    let empty = TokenStream::new();
    let mut input: Parser = empty.to_parser();
    assert!(input.parse::<Nothing>().is_ok());
    let tokens: TokenStream =
        (&[TokenTree::Leaf((t![some_token]).into(), Span::call_site())][..]).into();
    let mut input: Parser = tokens.to_parser();
    assert!(input.parse::<Nothing>().is_err());
}

#[test]
fn test_parse_ident() {
    let tokens: TokenStream = [
        TokenTree::Leaf(t![#my_ident], Span::call_site()),
        TokenTree::Leaf(t![#another_ident], Span::call_site()),
        TokenTree::Leaf(t![,], Span::call_site()),
    ][..]
        .into();
    let mut input: Parser = (&tokens).into();
    let a = Ident::parse(&mut input).unwrap();
    assert_eq!(a, "my_ident");
    let b: Ident = input.parse().unwrap();
    assert_eq!(b, "another_ident");
    let err = Ident::parse(&mut input).unwrap_err();
    assert_eq!(err.messages.first().unwrap(), "expected ident, found `,`");
}
