#![cfg(test)]

use quote::quote;
use sin::parsing::*;

#[test]
fn test_parser_basic() {
    let mut parser = Parser::<Stateless>::from(quote! {
        struct MyStruct {
            field1: u32,
            field2: usize,
        }
    });
    assert_eq!(parser.has_next(), true);
    assert_eq!(
        parser.peek(quote!(struct).into_iter().next().unwrap().into()),
        true
    );
}
