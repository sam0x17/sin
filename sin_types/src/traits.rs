use crate::*;

pub trait AsInStr {
    fn in_str(&self) -> InStr;
}

impl AsInStr for bool {
    fn in_str(&self) -> InStr {
        match self {
            true => "true".into(),
            false => "false".into(),
        }
    }
}

impl AsInStr for char {
    fn in_str(&self) -> InStr {
        self.to_string().into()
    }
}

impl AsInStr for InStr {
    fn in_str(&self) -> InStr {
        *self
    }
}
