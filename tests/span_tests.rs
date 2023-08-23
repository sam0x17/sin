#![cfg(test)]

use sin::*;
use test_macros::*;

#[test]
fn test_span_from_span1() {
    span_from_span1!(a);
    span_from_span1!({ a b c d e f g});
    span_from_span1! {
        {
            pub fn my_fn<T: Something>(&self, something: T) -> Option<T> {
                Some(something)
            }
        }
    };
}

#[test]
fn test_span_round_trip() {
    span_round_trip!(a b c d e f g);
    span_round_trip!({ a b c d e f g });
    span_round_trip! {
        {
            pub mod my_module {
                pub enum MyEnum<T> {
                    Red,
                    Blue(T),
                }
            }
        }
    }
}

#[test]
fn test_span_new_fallback() {
    let span = Span::new("pub fn foo() {}");
    assert!(matches!(span, S))
}
