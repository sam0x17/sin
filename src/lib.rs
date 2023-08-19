//! # Sin
//!
//! Sin (a perversion of dtolnay's [syn](https://crates.io/crates/syn) crate), aims to be a
//! compatible (via optional features), friendly alternative to the
//! [proc-macro2](https://crates.io/crates/proc-macro2) / [syn](https://crates.io/crates/syn) /
//! [quote](https://crates.io/crates/quote) ecosystem. The goal of sin is to provide all of the
//! same features, but without some of the limitations of `syn`. For example, `sin::parse` will
//! be implemented for _all_ underlying sin types without the need for awkward situational
//! macros such as `parenthesized!`, `parse_inner`, etc., and implementing `sin::parse` will
//! require implementing sin's equivalent of `quote::ToTokens`, meaning anything in the sin
//! ecosystem that can be parsed can also be turned back into tokens automatically.
//! Compatibility with the `proc-macro2` ecosystem will also be provided via feature-gated
//! `From<TokenStream2>` and `To<TokenStream2>` implementations.
//!
//! Where the `proc-macro2`/`syn` ecosystem aims to provide accurate+complete parsing of valid
//! Rust syntax, including complex structures like functions, impl blocks, etc, sin aims to
//! maximize the developer UX to make syntax parsing a joy, while at the same time catering to
//! those who wish to parse arbitrary syntax that may not necessarily be valid Rust code. Thus
//! sin aims to be the tool you reach for when you want to define and parse _custom syntax_ in
//! Rust proc and attribute macros, including implementing support for non-Rust grammars.
//!
//! Sin is a work in progress. The first usable version will be 0.1.0.
//!

pub mod parsing;
pub use parsing::*;

pub use sin_types as types;
pub use sin_types::*;

pub use sin_macros::*;
