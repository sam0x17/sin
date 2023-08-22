extern crate proc_macro;
use crate::InStr;
use core::{fmt::Debug, hash::Hash, ops::Deref};
use interned::{derive_from_interned_impl_value, unsafe_impl_data_type, Interned};
use proc_macro::Span as Span1;
use staticize::derive_staticize;

pub trait Span1Extensions: Sized {
    fn id(&self) -> u32 {
        unsafe { core::mem::transmute_copy(self) }
    }

    fn from_id(id: u32) -> Self {
        unsafe { core::mem::transmute_copy(&id) }
    }
}

impl Span1Extensions for Span1 {}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SpanStyle {
    Normal,
    CallSite,
    MixedSite,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SpanData {
    ProcMacro(u32),
    Fallback {
        style: SpanStyle,
        source_text: Option<InStr>,
    },
}

derive_staticize!(SpanData);
unsafe_impl_data_type!(SpanData, Value);
derive_from_interned_impl_value!(SpanData);

impl SpanData {
    /// Returns the internal compiler-assigned identifier for this [`SpanData`], if applicable.
    ///
    /// For a [`SpanData::ProcMacro`] this should return a [`Some`] value. For
    /// [`SpanData::Fallback`], this will return [`None`].
    pub fn span_id(&self) -> Option<u32> {
        match self {
            SpanData::ProcMacro(id) => Some(*id),
            SpanData::Fallback { .. } => None,
        }
    }
}

impl From<u32> for SpanData {
    fn from(value: u32) -> Self {
        SpanData::ProcMacro(value)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Span(Interned<SpanData>);

impl Deref for Span {
    type Target = SpanData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Span1> for Span {
    fn from(span1: Span1) -> Self {
        let data: SpanData = span1.id().into();
        Span(data.into())
    }
}

impl From<Span> for Span1 {
    fn from(span: Span) -> Self {
        match span.0.interned_value() {
            SpanData::ProcMacro(id) => Span1::from_id(*id),
            SpanData::Fallback { style, .. } => match style {
                SpanStyle::MixedSite => Span1::mixed_site(),
                SpanStyle::Normal | SpanStyle::CallSite => Span1::call_site(),
            },
        }
    }
}

impl Span {
    /// Returns the underlying [`SpanData`] used to represent this [`Span`].
    ///
    /// This is an internal type and should be used with caution.
    pub fn span_data(&self) -> &SpanData {
        self.0.interned_value()
    }

    /// The span of the invocation of the current procedural macro. Identifiers created with
    /// this span will be resolved as if they were written directly at the macro call location
    /// (call-site hygiene) and other code at the macro call site will be able to refer to them
    /// as well.
    pub fn call_site() -> Span {
        if proc_macro::is_available() {
            let data: SpanData = proc_macro::Span::call_site().id().into();
            Span(data.into())
        } else {
            Span(
                SpanData::Fallback {
                    style: SpanStyle::CallSite,
                    source_text: None,
                }
                .into(),
            )
        }
    }

    /// A span that represents `macro_rules` hygiene, and sometimes resolves at the macro
    /// definition site (local variables, labels, `$crate`) and sometimes at the macro call
    /// site (everything else). The span location is taken from the call-site.
    pub fn mixed_site() -> Span {
        if proc_macro::is_available() {
            let data: SpanData = proc_macro::Span::mixed_site().id().into();
            Span(data.into())
        } else {
            Span(
                SpanData::Fallback {
                    style: SpanStyle::MixedSite,
                    source_text: None,
                }
                .into(),
            )
        }
    }

    /// Returns the source text behind a span, if available. This preserves the original source
    /// code, including spaces and comments.
    pub fn source_text(&self) -> Option<InStr> {
        match self.0.interned_value() {
            SpanData::ProcMacro(id) => match Span1::from_id(*id).source_text() {
                Some(string) => Some(string.into()),
                None => None,
            },
            SpanData::Fallback { source_text, .. } => *source_text,
        }
    }
}
