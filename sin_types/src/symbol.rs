use core::fmt::Display;
use core::ops::Deref;
use interned::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Symbol(Interned<&'static str>);

impl Symbol {
    pub fn as_str(&self) -> &'static str {
        self.0.interned_str()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.interned_str())
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        self.0.interned_str()
    }
}

impl<'a> From<&'a str> for Symbol {
    fn from(value: &'a str) -> Self {
        Symbol(Interned::<&'static str>::from(value))
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Symbol(Interned::<&'static str>::from(value.as_str()))
    }
}

impl From<Interned<&'static str>> for Symbol {
    fn from(value: Interned<&'static str>) -> Self {
        Symbol(value)
    }
}

impl<'a> From<Symbol> for &'a str {
    fn from(value: Symbol) -> Self {
        value.0.interned_str()
    }
}

impl From<Symbol> for String {
    fn from(value: Symbol) -> Self {
        value.0.interned_str().to_string()
    }
}

impl PartialEq<&str> for Symbol {
    fn eq(&self, other: &&str) -> bool {
        self.0.interned_str().eq(*other)
    }
}

impl PartialEq<String> for Symbol {
    fn eq(&self, other: &String) -> bool {
        self.0.interned_str().eq(other.as_str())
    }
}

impl PartialOrd<&str> for Symbol {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.0.interned_str().partial_cmp(*other)
    }
}

impl Deref for Symbol {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.interned_str()
    }
}
