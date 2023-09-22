use crate::{span::Spanned, *};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenStream {
    tokens: Vec<TokenTree>,
    span: Span,
}

impl Spanned for TokenStream {
    fn span(&self) -> Span {
        self.span
    }
}

impl TokenStream {
    pub fn iter(&self) -> TSIterator {
        TSIterator {
            cursor: 0,
            tokens: &self.tokens,
            state: (),
        }
    }

    pub fn new() -> TokenStream {
        TokenStream {
            tokens: Vec::new(),
            span: Span::call_site(),
        }
    }

    pub fn from_tokens(tokens: &[TokenTree]) -> TokenStream {
        let mut ts = TokenStream::new();
        for token in tokens {
            ts.push(token);
        }
        ts
    }

    pub fn extend(&mut self, tokens: impl Into<TokenStream>) {
        let tokens = tokens.into();
        self.tokens.extend(tokens.tokens);
    }

    pub fn push(&mut self, token_tree: impl Into<TokenTree>) {
        self.tokens.push(token_tree.into());
    }
}

impl From<TokenTree> for TokenStream {
    fn from(value: TokenTree) -> Self {
        TokenStream::from_tokens(&[value])
    }
}

impl From<&[TokenTree]> for TokenStream {
    fn from(value: &[TokenTree]) -> Self {
        TokenStream::from_tokens(value)
    }
}

impl FromIterator<TokenTree> for TokenStream {
    fn from_iter<T: IntoIterator<Item = TokenTree>>(iter: T) -> Self {
        let v: Vec<TokenTree> = iter.into_iter().collect();
        let span = Span::new(InStr::from(
            v.iter().map(|t| t.as_str()).collect::<String>(),
        ));
        TokenStream { tokens: v, span }
    }
}

pub trait Peekable<T>: Iterator<Item = T> {
    fn peek_n(&self, n: isize) -> Option<T>;
    fn peek(&self) -> Option<T>;
}

pub struct TSIterator<'a, T = ()> {
    cursor: usize,
    tokens: &'a Vec<TokenTree>,
    /// Specified by `T`, can be optionally used to keep track of state information while
    /// iterating/parsing over a [`TokenStream`].
    pub state: T,
}

impl<'a> Iterator for TSIterator<'a> {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;
        if self.cursor >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.cursor].clone())
    }
}

impl<'a> TSIterator<'a> {
    pub fn peek_n(&self, n: isize) -> Option<TokenTree> {
        let idx = self.cursor as isize + n;
        if idx < 0 {
            return None;
        }
        self.tokens.get(idx as usize).cloned()
    }

    pub fn peek(&self) -> Option<TokenTree> {
        self.tokens.get(self.cursor + 1).cloned()
    }

    pub fn matches_peek(&self, _token: Token) -> bool {
        match self.peek() {
            Some(_peeked_token) => todo!(),
            None => todo!(),
        }
    }
}

impl<'a> From<&'a TokenStream> for TSIterator<'a> {
    fn from(value: &'a TokenStream) -> Self {
        value.iter()
    }
}
