use super::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Rep<T: Parse, S: Parse = NoOp, const COMPACT: bool = false> {
    items: Vec<T>,
    seps: Vec<S>,
    span: Span,
}

impl<T: Parse, S: Parse, const COMPACT: bool> Spanned for Rep<T, S, COMPACT> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T: Parse, S: Parse, const COMPACT: bool> ToTokens for Rep<T, S, COMPACT> {
    fn to_token_stream(&self) -> TokenStream {
        todo!()
    }
}

impl<T: Parse, S: Parse, const COMPACT: bool> Parse for Rep<T, S, COMPACT> {
    fn parse<'a, I: Default + Clone>(input: &mut Parser<'a, I>) -> ParseResult<Self> {
        let mut ret = Rep {
            items: Vec::new(),
            seps: Vec::new(),
            span: input.span(),
        };
        loop {
            if input.peek().is_none() {
                break;
            }
            if COMPACT && input.peek_parse::<T>().is_err() {
                break;
            }
            ret.items.push(input.parse::<T>()?);
            if input.peek().is_none() {
                break;
            }
            if COMPACT && input.peek_parse::<S>().is_err() {
                break;
            }
            ret.seps.push(input.parse::<S>()?);
        }
        ret.span = ret.span.join(input.span()).unwrap_or(Span::call_site());
        Ok(ret)
    }
}

impl<T: Parse, S: Parse> Rep<T, S> {
    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn separators(&self) -> &Vec<S> {
        &self.seps
    }

    pub fn punctuated(&self) -> Vec<(&T, Option<&S>)> {
        let mut ret = Vec::new();
        let mut seps_iter = self.seps.iter();
        for item in &self.items {
            ret.push((item, seps_iter.next()));
        }
        ret
    }
}

#[test]
fn test_parse_rep_path() {
    let tokens: TokenStream = [
        TokenTree::Leaf(t![#seg_1], Span::call_site()),
        TokenTree::Leaf(t![::], Span::call_site()),
        TokenTree::Leaf(t![#seg_2], Span::call_site()),
        TokenTree::Leaf(t![::], Span::call_site()),
        TokenTree::Leaf(t![#seg_3], Span::call_site()),
    ][..]
        .into();
    let rep = parse::<Rep<Ident, punct::PathSep>>(tokens).unwrap();
    assert_eq!(rep.items.len(), 3);
    assert_eq!(rep.seps.len(), 2);
    assert_eq!(rep.punctuated().len(), 3);
    let punctuated = rep.punctuated();
    let last = punctuated.last().unwrap();
    assert_eq!(*last.0, "seg_3");
    assert!(last.1.is_none());
    let first = punctuated.first().unwrap();
    assert_eq!(*first.1.unwrap(), punct::PathSep::default());
    assert_eq!(*first.0, "seg_1");
}
