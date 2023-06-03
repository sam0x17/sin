use proc_macro::{Delimiter, TokenStream, TokenTree};
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use sin_types::*;
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
            quote!(sin::Token::Ident(#ident.to_string()).into()).into()
        }
        TokenTree::Punct(punct) => {
            let st = punct.to_string();
            let punct: Punct = st.as_str().into();
            let dbg: TokenStream2 = format!("{:?}", punct).parse().unwrap();
            quote!(sin::Token::Punct(sin::Punct::#dbg)).into()
        }
        TokenTree::Literal(lit) => lit.to_string().parse().unwrap(),
    }
}
