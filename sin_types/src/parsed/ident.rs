use super::*;

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

#[test]
fn test_parse_tokens() {
    let tokens: TokenStream = [TokenTree::Leaf(t![#my_ident], Span::call_site())][..].into();
    assert!(Ident::parse_tokens(tokens).is_ok());
    let tokens: TokenStream = [
        TokenTree::Leaf(t![#my_ident], Span::call_site()),
        TokenTree::Leaf(t![#another_ident], Span::call_site()),
    ][..]
        .into();
    assert!(Ident::parse_tokens(tokens).is_err());
    let tokens: TokenStream = [TokenTree::Leaf(t![test], Span::call_site())][..].into();
    assert!(Ident::parse_tokens(tokens).is_err());
}
