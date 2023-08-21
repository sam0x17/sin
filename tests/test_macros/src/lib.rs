use proc_macro::Span as Span1;
use proc_macro::TokenStream;
use sin::span::*;

#[proc_macro]
pub fn span_from_span1(tokens: TokenStream) -> TokenStream {
    let mut iter = tokens.into_iter();
    while let Some(tt) = iter.next() {
        let span1 = tt.span();
        let span: Span = span1.into();
        let span_source_text: Option<String> = span.source_text().map(|st| st.into());
        assert_eq!(span_source_text, span1.source_text());
        assert_eq!(span.span_data().span_id(), Some(span1.id()));
    }
    "".parse().unwrap()
}

#[proc_macro]
pub fn span_round_trip(tokens: TokenStream) -> TokenStream {
    let mut iter = tokens.into_iter();
    while let Some(tt) = iter.next() {
        let span1 = tt.span();
        let span: Span = span1.into();
        let roundtrip: Span1 = span.into();
        assert_eq!(roundtrip.source_text(), span1.source_text());
        assert_eq!(Some(roundtrip.id()), span.span_data().span_id());
    }
    "".parse().unwrap()
}
