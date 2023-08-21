// use sin::*;
use sin::*;
use test_macros::*;

#[test]
fn test_span1_from_span() {
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
