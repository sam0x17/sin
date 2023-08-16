mod literal;
pub use literal::*;
mod token;
pub use token::*;
mod span;
pub use interned::InStr;
pub use span::*;

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
        T: Copy + Clone + PartialEq + Eq + PartialOrd + Ord + core::hash::Hash + core::fmt::Debug,
    {
    }
}

#[doc(hidden)]
pub mod __private {
    pub use litrs::Literal;
}
