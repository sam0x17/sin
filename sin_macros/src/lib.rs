// use proc_macro::TokenStream;
// use sin_types::*;

// fn escape_quote<S: AsRef<str>>(text: S) -> String {
//     text.as_ref().replace("\"", "\\\"")
// }

// fn compile_error<S: AsRef<str>>(message: S) -> TokenStream {
//     let message = escape_quote(message);
//     format!("compile_error!(\"{}\")", message).parse().unwrap()
// }

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
