extern crate alloc;
extern crate proc_macro;
use core::fmt::{self, Debug, Write};

use proc_macro::Span as Span1;
use symbol::Symbol;

struct StrBuffer<const N: usize> {
    buff: [u8; N],
    len: usize,
}

impl<const N: usize> Write for StrBuffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.len() + self.len > self.buff.len() {
            return Err(fmt::Error);
        }
        let bytes = s.as_bytes();
        self.buff[self.len..self.len + s.len()].copy_from_slice(bytes);
        self.len += s.len();
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        if self.len >= self.buff.len() {
            return Err(fmt::Error);
        }
        self.buff[self.len] = c as u8;
        self.len += 1;
        Ok(())
    }
}

impl<const N: usize> StrBuffer<N> {
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buff[0..self.len]).unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Span {
    Sin(Span2),
    ProcMacro(Span1),
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Hash)]
pub enum Span2 {
    Local {
        start: usize,
        len: usize,
        source_text: Symbol,
    },
    MixedSite,
    CallSite,
}

impl From<Span1> for Span {
    fn from(value: Span1) -> Self {
        Span::ProcMacro(value)
    }
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

impl From<Span2> for Span {
    fn from(value: Span2) -> Self {
        Span::Sin(value.clone())
    }
}

// pub trait SpanHack: Sized {
//     fn raw_bytes(&self) -> &[u8] {
//         let pointer: *const Self = self;
//         let pointer: *const u8 = pointer as *const u8;
//         unsafe { alloc::slice::from_raw_parts(pointer, core::mem::size_of::<Self>()) }
//     }
// }

pub trait SpanHack: Debug {
    fn debug_str(&self, buff: &mut StrBuffer<64>) {
        write!(buff, "{:#?}", self).unwrap();
    }
}

impl SpanHack for Span1 {}

impl Span {
    // /// The span of the invocation of the current procedural macro. Identifiers with this span
    // /// will be resolved as if they were written directly at the macro call location (call-site
    // /// hygiene) and other code at the macro call site will be able to refer to them as well.
    // pub fn call_site() -> Span {}
}
