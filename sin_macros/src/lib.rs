use proc_macro::{Delimiter, TokenStream, TokenTree};
use proc_macro2::Span;
use quote::quote;
use syn::Error;

#[proc_macro]
pub fn tt(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();
    let Some(token_tree) = tokens.next() else {
        return Error::new(
            Span::call_site(),
            "A valid `TokenTree` must be provided."
        ).to_compile_error().into()
    };
    if tokens.next().is_some() {
        return Error::new(Span::call_site(), "Only one `TokenTree` can be provided.")
            .to_compile_error()
            .into();
    }
    match token_tree {
        TokenTree::Group(group) => {
            if !group.stream().is_empty() {
                return Error::new(
                    Span::call_site(),
                    "No contents may be provided for `{}`, `[]`, or `()`.",
                )
                .to_compile_error()
                .into();
            } else {
                match group.delimiter() {
                    Delimiter::Parenthesis => quote!(sin::Token::GroupPunct::Paren).into(),
                    Delimiter::Brace => quote!(sin::Token::GroupPunct::Brace).into(),
                    Delimiter::Bracket => quote!(sin::Token::GroupPunct::Bracket).into(),
                    Delimiter::None => unimplemented!(),
                }
            }
        }
        TokenTree::Ident(ident) => {
            let ident = ident.to_string();
            quote!(sin::Token::Ident(#ident).into()).into()
        }
        TokenTree::Punct(punct) => todo!(),
        TokenTree::Literal(_) => todo!(),
    }
}
