use super::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Nothing;

impl ToTokens for Nothing {
    fn to_token_stream(&self) -> TokenStream {
        TokenStream::new()
    }
}

impl Spanned for Nothing {
    fn span(&self) -> Span {
        Span::call_site()
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
