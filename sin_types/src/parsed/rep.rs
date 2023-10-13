use super::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Rep<T: Parse, S: Parse = Nothing> {
    items: Vec<T>,
    seps: Vec<S>,
    span: Span,
}

impl<T: Parse, S: Parse> Spanned for Rep<T, S> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T: Parse, S: Parse> ToTokens for Rep<T, S> {
    fn to_token_stream(&self) -> TokenStream {
        todo!()
    }
}

impl<T: Parse, S: Parse> Parse for Rep<T, S> {
    fn parse<'a, I: Default + Clone>(input: &mut Parser<'a, I>) -> ParseResult<Self> {
        todo!()
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
