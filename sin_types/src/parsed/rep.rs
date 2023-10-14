use super::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Rep<T: Parse, S: Parse = NoOp, const COMPACT: bool = false> {
    items: Vec<T>,
    seps: Vec<S>,
    span: Span,
}

impl<T: Parse, S: Parse, const COMPACT: bool> Spanned for Rep<T, S, COMPACT> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T: Parse, S: Parse, const COMPACT: bool> ToTokens for Rep<T, S, COMPACT> {
    fn to_token_stream(&self) -> TokenStream {
        todo!()
    }
}

impl<T: Parse, S: Parse, const COMPACT: bool> Parse for Rep<T, S, COMPACT> {
    fn parse<'a, I: Default + Clone>(input: &mut Parser<'a, I>) -> ParseResult<Self> {
        let mut ret = Rep {
            items: Vec::new(),
            seps: Vec::new(),
            span: input.span(),
        };
        loop {
            if COMPACT {
                todo!();
            } else {
                let item = input.parse::<T>()?;
            }
        }
        Ok(ret)
    }
}

impl<T: Parse, S: Parse> Rep<T, S> {
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn separators(&self) -> &Vec<S> {
        &self.seps
    }

    pub fn punctuated(&self) -> Vec<(&T, Option<&S>)> {
        let mut ret = Vec::new();
        let mut seps_iter = self.seps.iter();
        for item in &self.items {
            ret.push((item, seps_iter.next()));
        }
        ret
    }
}
