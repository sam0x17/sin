use proc_macro::{Delimiter, TokenStream, TokenTree};
use sin_types::*;

fn escape_quote<S: AsRef<str>>(text: S) -> String {
    text.as_ref().replace("\"", "\\\"")
}

fn compile_error<S: AsRef<str>>(message: S) -> TokenStream {
    let message = escape_quote(message);
    format!("compile_error!(\"{}\")", message).parse().unwrap()
}

fn simple_quote<S: AsRef<str>>(code: S) -> TokenStream {
    code.as_ref().parse().unwrap()
}

#[proc_macro]
pub fn tt(tokens: TokenStream) -> TokenStream {
    let mut tokens = tokens.into_iter();
    let Some(token_tree) = tokens.next() else {
        return compile_error("A valid `TokenTree` must be provided.");
    };
    if tokens.next().is_some() {
        return compile_error("Only one `TokenTree` can be provided.");
    }
    match token_tree {
        TokenTree::Group(group) => {
            if !group.stream().is_empty() {
                return compile_error("No contents may be provided for `{}`, `[]`, or `()`.");
            } else {
                match group.delimiter() {
                    Delimiter::Parenthesis => simple_quote("sin::Token::GroupPunct::Paren"),
                    Delimiter::Brace => simple_quote("sin::Token::GroupPunct::Brace"),
                    Delimiter::Bracket => simple_quote("sin::Token::GroupPunct::Bracket"),
                    Delimiter::None => unimplemented!(),
                }
            }
        }
        TokenTree::Ident(ident) => simple_quote(format!(
            "sin::Token::Ident(\"{}\".to_string())",
            ident.to_string()
        ))
        .into(),
        TokenTree::Punct(punct) => {
            let punct: Punct = punct.to_string().as_str().into();
            simple_quote(format!("sin::Token::Punct(sin::Punct::{:?})", punct)).into()
        }
        TokenTree::Literal(lit) => lit.to_string().parse().unwrap(),
    }
}
