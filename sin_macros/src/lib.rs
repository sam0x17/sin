// use litrs::Literal;
// use proc_macro::{TokenStream, TokenTree};

// fn escape_quote<S: AsRef<str>>(text: S) -> String {
//     text.as_ref().replace("\"", "\\\"")
// }

// fn compile_error<S: AsRef<str>>(message: S) -> TokenStream {
//     let message = escape_quote(message);
//     format!("compile_error!(\"{}\")", message).parse().unwrap()
// }

// #[proc_macro]
// pub fn literal_arm(tokens: TokenStream) -> TokenStream {
//     let mut iter = tokens.into_iter();
//     let Some(token) = iter.next() else { return compile_error("expected exactly one token") };

//     TokenStream::new()
// }

// use sin_types::*;

// fn simple_quote<S: AsRef<str>>(code: S) -> TokenStream {
//     code.as_ref().parse().unwrap()
// }

// #[proc_macro]
// pub fn test_backend(tokens: TokenStream) -> TokenStream {
//     let mut iter = tokens.into_iter();
//     let span = iter.next().unwrap().span();
//     println!("{:#?}", span);
//     println!("{}", span.source_text().unwrap());
//     TokenStream::new()
// }
