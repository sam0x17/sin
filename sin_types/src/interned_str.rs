use core::fmt::Display;
use core::ops::Deref;
use interned::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct InStr(Interned<&'static str>);

impl InStr {
    pub fn as_str(&self) -> &'static str {
        self.0.interned_str()
    }
}

impl Display for InStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.interned_str())
    }
}

impl AsRef<str> for InStr {
    fn as_ref(&self) -> &str {
        self.0.interned_str()
    }
}

impl<'a> From<&'a str> for InStr {
    fn from(value: &'a str) -> Self {
        InStr(Interned::<&'static str>::from(value))
    }
}

impl From<String> for InStr {
    fn from(value: String) -> Self {
        InStr(Interned::<&'static str>::from(value.as_str()))
    }
}

impl From<Interned<&'static str>> for InStr {
    fn from(value: Interned<&'static str>) -> Self {
        InStr(value)
    }
}

impl<'a> From<InStr> for &'a str {
    fn from(value: InStr) -> Self {
        value.0.interned_str()
    }
}

impl From<InStr> for String {
    fn from(value: InStr) -> Self {
        value.0.interned_str().to_string()
    }
}

impl PartialEq<&str> for InStr {
    fn eq(&self, other: &&str) -> bool {
        self.0.interned_str().eq(*other)
    }
}

impl PartialEq<String> for InStr {
    fn eq(&self, other: &String) -> bool {
        self.0.interned_str().eq(other.as_str())
    }
}

impl PartialOrd<&str> for InStr {
    fn partial_cmp(&self, other: &&str) -> Option<std::cmp::Ordering> {
        self.0.interned_str().partial_cmp(*other)
    }
}

impl Deref for InStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.interned_str()
    }
}
