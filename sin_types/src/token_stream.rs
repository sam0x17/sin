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
        }
    }
}

pub struct TSIterator<'a> {
    cursor: usize,
    tokens: &'a Vec<TokenTree>,
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
}
