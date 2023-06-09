extern crate proc_macro;

use crate::Symbol;
use core::fmt::Debug;
use proc_macro::Span as Span1;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    span: Span2,
}

#[derive(Clone, Debug, Copy)]
enum Span2 {
    ProcMacro(SpanData),
    Local(SpanData),
    MixedSite,
    CallSite,
}

impl From<Span1> for Span {
    fn from(value: Span1) -> Self {
        Span {
            span: Span2::ProcMacro(value.span_data()),
        }
    }
}

impl From<&Span1> for Span {
    fn from(value: &Span1) -> Self {
        Span {
            span: Span2::ProcMacro(value.span_data()),
        }
    }
}

impl From<&Span2> for Span {
    fn from(value: &Span2) -> Self {
        Span { span: *value }
    }
}

impl From<Span2> for Span {
    fn from(value: Span2) -> Self {
        Span { span: value }
    }
}

impl From<Span> for Span1 {
    fn from(value: Span) -> Self {
        match value.span {
            Span2::MixedSite => Span1::mixed_site(),
            Span2::ProcMacro(_span) => todo!(),
            _ => Span1::call_site(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SpanData {
    pub start: usize,
    pub len: usize,
    pub source_text: Symbol,
}

pub trait ProvideSpanData {
    fn span_data(&self) -> SpanData;
}

impl ProvideSpanData for Span1 {
    fn span_data(&self) -> SpanData {
        todo!()
    }
}

pub trait SpanExtensions {
    fn unique_data(&self) -> String;
}

impl SpanExtensions for Span1 {
    fn unique_data(&self) -> String {
        format!("{self:#?}:{source:#?}", source = self.source_text())
    }
}

impl Span {
    /// The span of the invocation of the current procedural macro. Identifiers created with
    /// this span will be resolved as if they were written directly at the macro call location
    /// (call-site hygiene) and other code at the macro call site will be able to refer to them
    /// as well.
    pub fn call_site() -> Span {
        Span {
            span: Span2::CallSite,
        }
    }

    /// A span that represents `macro_rules` hygiene, and sometimes resolves at the macro
    /// definition site (local variables, labels, `$crate`) and sometimes at the macro call
    /// site (everything else). The span location is taken from the call-site.
    pub fn mixed_site() -> Span {
        Span {
            span: Span2::MixedSite,
        }
    }

    /// Returns the source text behind a span. This preserves the original source code,
    /// including spaces and comments.
    ///
    /// [`None`] is returned if the backing is a [`proc_macro::Span`] and the span itself does
    /// not refer to real source code or if this is a [`mixed_site`](`Span::mixed_site`) or
    /// [`call_site`](`Span::call_site`) span.
    pub fn source_text(&self) -> Option<Symbol> {
        match self.span {
            Span2::Local(SpanData {
                start,
                len,
                source_text,
            }) => Some(source_text[start..(start + len)].into()),
            Span2::MixedSite | Span2::CallSite => None,
            Span2::ProcMacro(span) => Some(span.source_text),
        }
    }

    /// Creates a new [`Span`] from the specified source [`str`],
    /// [`String`](`alloc::string::String`), or [`Symbol`] (or anything that implements
    /// [`Into<Symbol>`]). Calling this with an already allocated [`Symbol`] is a zero-cost
    /// operation.
    pub fn from_source<S: Into<Symbol>>(source: S) -> Span {
        let source = source.into();
        Span {
            span: Span2::Local(SpanData {
                start: 0,
                len: source.len(),
                source_text: source,
            }),
        }
    }
}
