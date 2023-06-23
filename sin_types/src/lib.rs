#![no_std]

mod interned_bytes;
pub use interned_bytes::*;
mod literal;
pub use literal::*;
mod token;
pub use token::*;
mod span;
pub use span::Span;
pub use span::*;

pub use symbol::Symbol;

pub mod util {
    pub const fn assert_sync<T>()
    where
        T: Sync,
    {
    }

    pub const fn assert_send<T>()
    where
        T: Sync,
    {
    }

    pub const fn assert_golden_traits<T>()
    where
        T: Send
            + Sync
            + Clone
            + Copy
            + PartialEq
            + Eq
            + PartialOrd
            + Ord
            + core::hash::Hash
            + core::fmt::Display
            + core::fmt::Debug,
    {
    }
}

#[doc(hidden)]
pub mod __private {
    pub use litrs::Literal;
}
