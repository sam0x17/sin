use crate::*;

pub struct TokenStream {
    tokens: Vec<Token>,
}

// dual fallback model?

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
    tokens: &'a Vec<Token>,
}

impl<'a> Iterator for TSIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;
        if self.cursor >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.cursor])
    }
}

impl<'a> TSIterator<'a> {
    pub fn peek_n(&self, n: isize) -> Option<Token> {
        let idx = self.cursor as isize + n;
        if idx < 0 {
            return None;
        }
        self.tokens.get(idx as usize).cloned()
    }

    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(self.cursor + 1).cloned()
    }
}
