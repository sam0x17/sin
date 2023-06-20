extern crate alloc;
extern crate proc_macro;

use alloc::string::String;
use core::ops::Range;
use proc_macro::Span as Span1;

#[derive(Clone, Debug)]
pub enum Span {
    Sin(Span2),
    ProcMacro(Span1),
}

#[derive(Clone, Debug)]
pub struct Span2 {
    range: Range<usize>,
    source_text: String,
}

impl From<&Span1> for Span {
    fn from(value: &Span1) -> Self {
        Span::ProcMacro(*value)
    }
}

impl From<&Span2> for Span {
    fn from(value: &Span2) -> Self {
        Span::Sin(value.clone())
    }
}

trait SpanHack: Sized {
    fn raw_bytes(&self) -> &[u8] {
        let pointer: *const Self = self;
        let pointer: *const u8 = pointer as *const u8;
        unsafe { alloc::slice::from_raw_parts(pointer, core::mem::size_of::<Self>()) }
    }
}

impl SpanHack for Span1 {}

impl Span {
    // /// The span of the invocation of the current procedural macro. Identifiers with this span
    // /// will be resolved as if they were written directly at the macro call location (call-site
    // /// hygiene) and other code at the macro call site will be able to refer to them as well.
    // pub fn call_site() -> Span {}
}
