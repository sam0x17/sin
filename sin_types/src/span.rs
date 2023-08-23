extern crate proc_macro;
use crate::InStr;
use core::{fmt::Debug, hash::Hash, ops::Deref};
use interned::{derive_from_interned_impl_value, unsafe_impl_data_type, Interned};
use proc_macro::Span as Span1;
use staticize::derive_staticize;

pub trait Span1Extensions: Sized {
    /// Returns the internal identifier used to identify this [`proc_macro::Span`] in the
    /// global interning table.
    ///
    /// Every [`proc_macro::Span`] has a unique identifier, including individual call sites.
    /// This is used as the basis for equality and uniqueness in [`Span`] (when not using the
    /// fallback implementation).
    fn id(&self) -> u32 {
        unsafe { core::mem::transmute_copy(self) }
    }

    /// Creates a new [`proc_macro::Span`] from a [`proc_macro::Span`] internal identifier.
    ///
    /// This is UB if the identifier is from a no longer active proc macro or doesn't
    /// correspond with a real span in the current proc macro input. If you use this method,
    /// the input should typically come from calling [`id`](`Span1Extensions::id`) on a
    /// [`proc_macro::Span`] unless you know what you are doing.
    ///
    /// Unfortunately the way [`proc_macro`] internals are currently implemented, you will
    /// experience an ICE if you try to do anything with a [`proc_macro::Span`] from a
    /// no-longer-active proc macro, and that applies to this method as well.
    ///
    /// Normally you shouldn't need to use this function but it is used in some [`Span`]
    /// internals and is provided as a convenience method and analogue to
    /// [`id`](`Span1Extensions::id`).
    unsafe fn from_id(id: u32) -> Self {
        unsafe { core::mem::transmute_copy(&id) }
    }
}

impl Span1Extensions for Span1 {}

/// An internal implementation detail of [`SpanData`] that delineates the different types of
/// [`Span`].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SpanStyle {
    /// This [`Span`] will exhibit "normal" resolution if it is emitted as output from a proc
    /// macro.
    Normal,
    /// This [`Span`] will exhibit call-site resolution if it is emitted as output from a proc
    /// macro.
    CallSite,
    /// This [`Span] will exhibit mixed-site / decl-macro style resolution if it is emitted as
    /// output from a proc macro.
    MixedSite,
}

/// An internal implementation detail of [`Span`] that contains the actual data that gets
/// interned on behalf of a particular [`Span`].
///
/// Returned by [`Span::span_data`].
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
            SpanData::ProcMacro(id) => unsafe { Span1::from_id(*id) },
            SpanData::Fallback { style, .. } => match style {
                SpanStyle::MixedSite => Span1::mixed_site(),
                SpanStyle::Normal | SpanStyle::CallSite => Span1::call_site(),
            },
        }
    }
}

impl From<SpanData> for Span {
    fn from(value: SpanData) -> Self {
        Span(value.into())
    }
}

impl From<Span> for SpanData {
    fn from(value: Span) -> Self {
        *value.span_data()
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
            SpanData::ProcMacro(id) => match unsafe { Span1::from_id(*id) }.source_text() {
                Some(string) => Some(string.into()),
                None => None,
            },
            SpanData::Fallback { source_text, .. } => *source_text,
        }
    }

    /// Creates a new `Span` from the specified source string.
    ///
    /// The created span will use the fallback implementation rather than a built-in
    /// [`proc_macro::Span`]. If `source` exactly matches an existing `TokenStream`, the
    /// created [`Span`] will be identical to / interchangeable with that `TokenStream`'s
    /// [`Span`].
    pub fn new(source: impl Into<InStr>) -> Span {
        Span(
            SpanData::Fallback {
                style: SpanStyle::Normal,
                source_text: Some(source.into()),
            }
            .into(),
        )
    }

    /// Returns `true` if this [`Span`] is using the fallback implementation rather than [`proc_macro::Span`].
    ///
    /// [`Span`]s created manually from input that does not originate from proc macro input
    /// always use the fallback implementation.
    pub fn is_fallback(&self) -> bool {
        let data: SpanData = *self.span_data();
        matches!(data, SpanData::Fallback { .. })
    }

    /// Converts this [`Span`] to use the fallback implementation rather than
    /// [`proc_macro::Span`], if it isn't already using the fallback implementation.
    ///
    /// Note that this is a destructive operation when used on a non-fallback [`Span`], since
    /// resolution of the span within the source proc macro input will be lost.
    pub fn to_fallback(&self) -> Span {
        if let SpanData::ProcMacro(id) = self.span_data() {
            let span1 = unsafe { Span1::from_id(*id) };
            SpanData::Fallback {
                style: SpanStyle::Normal,
                source_text: span1.source_text().map(|s| s.into()),
            }
            .into()
        } else {
            return *self;
        }
    }

    /// Creates a new [`Span`] from a [`proc_macro::Span`] internal identifier.
    ///
    /// This is UB if the identifier is from a no longer active proc macro or doesn't
    /// correspond with a real span in the current proc macro input. If you use this method,
    /// the input should typically come from calling [`id`](`Span1Extensions::id`) on a
    /// [`proc_macro::Span`] unless you know what you are doing.
    ///
    /// Unfortunately the way [`proc_macro`] internals are currently implemented, you will
    /// experience an ICE if you try to do anything with a [`proc_macro::Span`] from a
    /// no-longer-active proc macro, and that applies to this method as well.
    ///
    /// Normally you shouldn't need to use this function but it is used in some [`Span`]
    /// internals and is provided as a convenience method and analogue to
    /// [`id`](`Span1Extensions::id`).
    pub unsafe fn from_id(id: u32) -> Span {
        Span1::from_id(id).into()
    }
}
