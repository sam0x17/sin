use crate::{InPath, InStr};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SourceType {
    File(InPath),
    StringInput,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SourceData {
    ProcMacro,
    Fallback(InStr),
}
