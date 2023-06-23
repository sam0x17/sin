extern crate alloc;

use crate::{InternedBytes, Symbol};
use core::{
    hash::{Hash, Hasher},
    ops::Deref,
};
use litrs::ParseError;

pub trait ParseLiteral: Sized {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, ParseError>;
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct IntLit {
    pub raw: Symbol,
    pub lit: litrs::IntegerLit<&'static str>,
}

impl ParseLiteral for IntLit {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, ParseError> {
        let raw = Symbol::from(input.as_ref());
        let lit = litrs::IntegerLit::parse(raw.as_str())?;
        Ok(IntLit { raw, lit })
    }
}

impl PartialOrd for IntLit {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl Ord for IntLit {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl core::hash::Hash for IntLit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.raw.as_bytes());
    }
}

impl Deref for IntLit {
    type Target = litrs::IntegerLit<&'static str>;

    fn deref(&self) -> &Self::Target {
        &self.lit
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FloatLit {
    pub raw: Symbol,
    pub lit: litrs::FloatLit<&'static str>,
}

impl ParseLiteral for FloatLit {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, ParseError> {
        let raw = Symbol::from(input.as_ref());
        let lit = litrs::FloatLit::parse(raw.as_str())?;
        Ok(FloatLit { raw, lit })
    }
}

impl PartialOrd for FloatLit {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl Ord for FloatLit {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl core::hash::Hash for FloatLit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.raw.as_bytes());
    }
}

impl Deref for FloatLit {
    type Target = litrs::FloatLit<&'static str>;

    fn deref(&self) -> &Self::Target {
        &self.lit
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ByteLit {
    pub raw: Symbol,
    pub lit: litrs::ByteLit<&'static str>,
}

impl ParseLiteral for ByteLit {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, ParseError> {
        let raw = Symbol::from(input.as_ref());
        let lit = litrs::ByteLit::parse(raw.as_str())?;
        Ok(ByteLit { raw, lit })
    }
}

impl PartialOrd for ByteLit {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl Ord for ByteLit {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl core::hash::Hash for ByteLit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.raw.as_bytes());
    }
}

impl Deref for ByteLit {
    type Target = litrs::ByteLit<&'static str>;

    fn deref(&self) -> &Self::Target {
        &self.lit
    }
}

pub struct ByteStringLit {
    pub raw: Symbol,
    pub value: InternedBytes,
}

impl ParseLiteral for char {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, ParseError> {
        let raw = Symbol::from(input.as_ref());
        let lit = litrs::CharLit::parse(raw.as_str())?;
        Ok(lit.value())
    }
}

impl ParseLiteral for bool {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, ParseError> {
        let raw = Symbol::from(input.as_ref());
        let lit = litrs::BoolLit::parse(raw.as_str())?;
        Ok(lit.value())
    }
}

impl ParseLiteral for Symbol {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, ParseError> {
        let raw = Symbol::from(input.as_ref());
        let lit = litrs::StringLit::parse(raw.as_str())?;
        Ok(Symbol::from(lit.value()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Literal {
    Bool(bool),
    Char(char),
    Integer(IntLit),
    Float(FloatLit),
    String(Symbol),
    Byte(ByteLit),
    ByteString(InternedBytes),
}

impl ParseLiteral for Literal {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, ParseError> {
        let sym = Symbol::from(input.as_ref());
        match litrs::Literal::parse(sym.as_str()) {
            Ok(litrs::Literal::Bool(lit)) => Ok(Literal::Bool(lit.value())),
            Ok(litrs::Literal::Char(lit)) => Ok(Literal::Char(lit.value())),
            Ok(litrs::Literal::String(lit)) => Ok(Literal::String(Symbol::from(lit.value()))),
            Ok(litrs::Literal::Integer(lit)) => Ok(Literal::Integer(IntLit { raw: sym, lit })),
            Ok(litrs::Literal::Float(lit)) => Ok(Literal::Float(FloatLit { raw: sym, lit })),
            Ok(litrs::Literal::Byte(lit)) => Ok(Literal::Byte(ByteLit { raw: sym, lit })),
            Ok(litrs::Literal::ByteString(lit)) => {
                Ok(Literal::ByteString(InternedBytes::from(lit.value())))
            }
            Err(err) => Err(err),
        }
    }
}
