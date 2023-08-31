// use std::str::FromStr;

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Source {
    source_type: SourceType,
    data: SourceData,
}

// impl FromStr for Source {
//     type Err;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }
