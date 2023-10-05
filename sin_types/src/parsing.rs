use crate::{
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
            None => format!("expected {expected}").into(),
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
}

impl<'a, T: Default + Clone> From<TSIterator<'a, T>> for Parser<'a, T> {
    fn from(value: TSIterator<'a, T>) -> Self {
        Parser(value, Span::call_site())
    }
}

impl<'a, T: Default + Clone> From<&'a TokenStream> for Parser<'a, T> {
    fn from(value: &'a TokenStream) -> Self {
        Parser(value.iter_with_state(), Span::call_site())
    }
}

pub trait Parse: Sized {
    fn parse<'a, T: Default + Clone>(input: &mut Parser<'a, T>) -> ParseResult<Self>;
}

pub struct Ident {
    span: Span,
    ident: InStr,
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

#[test]
fn test_parse_ident() {
    let tokens: TokenStream = [TokenTree::Leaf(t![#my_ident], Span::call_site())][..].into();
    let mut input: Parser = (&tokens).into();
    let a: Ident = Ident::parse(&mut input).unwrap();
}
