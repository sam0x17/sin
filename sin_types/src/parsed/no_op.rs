use super::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct NoOp;

impl Spanned for NoOp {
    fn span(&self) -> Span {
        Span::call_site()
    }
}

impl ToTokens for NoOp {
    fn to_token_stream(&self) -> TokenStream {
        TokenStream::new()
    }
}

impl Parse for NoOp {
    fn parse<'a, T: Default + Clone>(_input: &mut Parser<'a, T>) -> ParseResult<Self> {
        Ok(NoOp)
    }
}
