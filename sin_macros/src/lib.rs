// use proc_macro::{TokenStream, TokenTree};

// fn escape_quote<S: AsRef<str>>(text: S) -> String {
//     text.as_ref().replace("\"", "\\\"")
// }

// fn compile_error<S: AsRef<str>>(message: S) -> TokenStream {
//     let message = escape_quote(message);
//     format!("compile_error!(\"{}\")", message).parse().unwrap()
// }
