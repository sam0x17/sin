use interned::Interned;

use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[must_use]
pub enum Pattern<T> {
    Specific(T),
    Wildcard,
}

impl<T: PartialEq> Matches<Pattern<T>> for T {
    fn matches(&self, pattern: Pattern<T>) -> bool {
        match pattern {
            Pattern::Specific(val) => *self == val,
            Pattern::Wildcard => true,
        }
    }
}

/// Generic trait representing anything that can be subjected to matching via a [`Pattern`].
pub trait Matches<T> {
    /// Returns `true` if `self` matches the specified `pattern`.
    fn matches(&self, pattern: T) -> bool;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TokenPattern {
    Ident(Pattern<InStr>),
    Literal(LiteralPattern),
    Delimiter(Pattern<Delimiter>),
    Punct(Pattern<Punct>),
    Keyword(Pattern<Keyword>),
    CustomKeyword(Pattern<InStr>),
}

impl Matches<TokenPattern> for Token {
    fn matches(&self, pattern: TokenPattern) -> bool {
        match (self, pattern) {
            (Token::Ident(ident), TokenPattern::Ident(pat)) => ident.matches(pat),
            (Token::Literal(lit), TokenPattern::Literal(pat)) => lit.matches(pat),
            (Token::Delimiter(delim), TokenPattern::Delimiter(pat)) => delim.matches(pat),
            (Token::Punct(punct), TokenPattern::Punct(pat)) => punct.matches(pat),
            (Token::Keyword(kw), TokenPattern::Keyword(pat)) => kw.matches(pat),
            (Token::CustomKeyword(ckw), TokenPattern::CustomKeyword(pat)) => ckw.matches(pat),
            _ => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LiteralPattern {
    BoolLit(Pattern<bool>),
    Char(Pattern<char>),
    Integer(Pattern<IntLit>),
    Float(Pattern<FloatLit>),
    String(Pattern<InStr>),
    Byte(Pattern<ByteLit>),
    ByteString(Pattern<ByteStringLit>),
}

impl ParseLiteral for LiteralPattern {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, litrs::ParseError> {
        let sym = InStr::from(input.as_ref());
        match litrs::Literal::parse(sym.as_str()) {
            Ok(litrs::Literal::Bool(lit)) => {
                Ok(LiteralPattern::BoolLit(Pattern::Specific(lit.value())))
            }
            Ok(litrs::Literal::Char(lit)) => {
                Ok(LiteralPattern::Char(Pattern::Specific(lit.value())))
            }
            Ok(litrs::Literal::String(lit)) => Ok(LiteralPattern::String(Pattern::Specific(
                InStr::from(lit.value()),
            ))),
            Ok(litrs::Literal::Integer(lit)) => {
                Ok(LiteralPattern::Integer(Pattern::Specific(IntLit {
                    raw: sym,
                    lit,
                })))
            }
            Ok(litrs::Literal::Float(lit)) => {
                Ok(LiteralPattern::Float(Pattern::Specific(FloatLit {
                    raw: sym,
                    lit,
                })))
            }
            Ok(litrs::Literal::Byte(lit)) => Ok(LiteralPattern::Byte(Pattern::Specific(ByteLit {
                raw: sym,
                lit,
            }))),
            Ok(litrs::Literal::ByteString(lit)) => {
                let value: Interned<&[u8]> = Interned::from(lit.value());
                Ok(LiteralPattern::ByteString(Pattern::Specific(
                    ByteStringLit {
                        raw: sym,
                        value,
                        is_raw_byte_string: lit.is_raw_byte_string(),
                    },
                )))
            }
            Err(err) => Err(err),
        }
    }
}

impl Matches<LiteralPattern> for Literal {
    fn matches(&self, pattern: LiteralPattern) -> bool {
        match (self, pattern) {
            (Literal::Bool(b), LiteralPattern::BoolLit(pat)) => b.matches(pat),
            (Literal::Char(c), LiteralPattern::Char(pat)) => c.matches(pat),
            (Literal::Integer(i), LiteralPattern::Integer(pat)) => i.matches(pat),
            (Literal::Float(f), LiteralPattern::Float(pat)) => f.matches(pat),
            (Literal::String(s), LiteralPattern::String(pat)) => s.matches(pat),
            (Literal::Byte(b), LiteralPattern::Byte(pat)) => b.matches(pat),
            (Literal::ByteString(b), LiteralPattern::ByteString(pat)) => b.matches(pat),
            _ => false,
        }
    }
}

#[rustfmt::skip]
#[macro_export]
macro_rules! pat {
    (abstract)       => { $crate::TokenPattern::Keyword($crate::Keyword::Abstract) };
    (as)             => { $crate::TokenPattern::Keyword($crate::Keyword::As) };
    (async)          => { $crate::TokenPattern::Keyword($crate::Keyword::Async) };
    (auto)           => { $crate::TokenPattern::Keyword($crate::Keyword::Auto) };
    (await)          => { $crate::TokenPattern::Keyword($crate::Keyword::Await) };
    (become)         => { $crate::TokenPattern::Keyword($crate::Keyword::Become) };
    (box)            => { $crate::TokenPattern::Keyword($crate::Keyword::Box) };
    (break)          => { $crate::TokenPattern::Keyword($crate::Keyword::Break) };
    (const)          => { $crate::TokenPattern::Keyword($crate::Keyword::Const) };
    (continue)       => { $crate::TokenPattern::Keyword($crate::Keyword::Continue) };
    (crate)          => { $crate::TokenPattern::Keyword($crate::Keyword::Crate) };
    (default)        => { $crate::TokenPattern::Keyword($crate::Keyword::Default) };
    (do)             => { $crate::TokenPattern::Keyword($crate::Keyword::Do) };
    (dyn)            => { $crate::TokenPattern::Keyword($crate::Keyword::Dyn) };
    (else)           => { $crate::TokenPattern::Keyword($crate::Keyword::Else) };
    (enum)           => { $crate::TokenPattern::Keyword($crate::Keyword::Enum) };
    (extern)         => { $crate::TokenPattern::Keyword($crate::Keyword::Extern) };
    (final)          => { $crate::TokenPattern::Keyword($crate::Keyword::Final) };
    (fn)             => { $crate::TokenPattern::Keyword($crate::Keyword::Fn) };
    (for)            => { $crate::TokenPattern::Keyword($crate::Keyword::For) };
    (if)             => { $crate::TokenPattern::Keyword($crate::Keyword::If) };
    (impl)           => { $crate::TokenPattern::Keyword($crate::Keyword::Impl) };
    (in)             => { $crate::TokenPattern::Keyword($crate::Keyword::In) };
    (let)            => { $crate::TokenPattern::Keyword($crate::Keyword::Let) };
    (loop)           => { $crate::TokenPattern::Keyword($crate::Keyword::Loop) };
    (macro)          => { $crate::TokenPattern::Keyword($crate::Keyword::Macro) };
    (match)          => { $crate::TokenPattern::Keyword($crate::Keyword::Match) };
    (mod)            => { $crate::TokenPattern::Keyword($crate::Keyword::Mod) };
    (move)           => { $crate::TokenPattern::Keyword($crate::Keyword::Move) };
    (mut)            => { $crate::TokenPattern::Keyword($crate::Keyword::Mut) };
    (override)       => { $crate::TokenPattern::Keyword($crate::Keyword::Override) };
    (priv)           => { $crate::TokenPattern::Keyword($crate::Keyword::Priv) };
    (pub)            => { $crate::TokenPattern::Keyword($crate::Keyword::Pub) };
    (ref)            => { $crate::TokenPattern::Keyword($crate::Keyword::Ref) };
    (return)         => { $crate::TokenPattern::Keyword($crate::Keyword::Return) };
    (Self)           => { $crate::TokenPattern::Keyword($crate::Keyword::SelfType) };
    (self)           => { $crate::TokenPattern::Keyword($crate::Keyword::SelfValue) };
    (static)         => { $crate::TokenPattern::Keyword($crate::Keyword::Static) };
    (struct)         => { $crate::TokenPattern::Keyword($crate::Keyword::Struct) };
    (super)          => { $crate::TokenPattern::Keyword($crate::Keyword::Super) };
    (trait)          => { $crate::TokenPattern::Keyword($crate::Keyword::Trait) };
    (try)            => { $crate::TokenPattern::Keyword($crate::Keyword::Try) };
    (type)           => { $crate::TokenPattern::Keyword($crate::Keyword::Type) };
    (typeof)         => { $crate::TokenPattern::Keyword($crate::Keyword::Typeof) };
    (union)          => { $crate::TokenPattern::Keyword($crate::Keyword::Union) };
    (unsafe)         => { $crate::TokenPattern::Keyword($crate::Keyword::Unsafe) };
    (unsized)        => { $crate::TokenPattern::Keyword($crate::Keyword::Unsized) };
    (use)            => { $crate::TokenPattern::Keyword($crate::Keyword::Use) };
    (virtual)        => { $crate::TokenPattern::Keyword($crate::Keyword::Virtual) };
    (where)          => { $crate::TokenPattern::Keyword($crate::Keyword::Where) };
    (while)          => { $crate::TokenPattern::Keyword($crate::Keyword::While) };
    (yield)          => { $crate::TokenPattern::Keyword($crate::Keyword::Yield) };
    (&)              => { $crate::TokenPattern::Punct($crate::Punct::And) };
    (&&)             => { $crate::TokenPattern::Punct($crate::Punct::AndAnd) };
    (&=)             => { $crate::TokenPattern::Punct($crate::Punct::AndEq) };
    (@)              => { $crate::TokenPattern::Punct($crate::Punct::At) };
    (^)              => { $crate::TokenPattern::Punct($crate::Punct::Caret) };
    (^=)             => { $crate::TokenPattern::Punct($crate::Punct::CaretEq) };
    (:)              => { $crate::TokenPattern::Punct($crate::Punct::Colon) };
    (,)              => { $crate::TokenPattern::Punct($crate::Punct::Comma) };
    ($)              => { $crate::TokenPattern::Punct($crate::Punct::Dollar) };
    (.)              => { $crate::TokenPattern::Punct($crate::Punct::Dot) };
    (..)             => { $crate::TokenPattern::Punct($crate::Punct::DotDot) };
    (...)            => { $crate::TokenPattern::Punct($crate::Punct::DotDotDot) };
    (..=)            => { $crate::TokenPattern::Punct($crate::Punct::DotDotEq) };
    (=)              => { $crate::TokenPattern::Punct($crate::Punct::Eq) };
    (==)             => { $crate::TokenPattern::Punct($crate::Punct::EqEq) };
    (=>)             => { $crate::TokenPattern::Punct($crate::Punct::FatArrow) };
    (>=)             => { $crate::TokenPattern::Punct($crate::Punct::Ge) };
    (>)              => { $crate::TokenPattern::Punct($crate::Punct::Gt) };
    (<-)             => { $crate::TokenPattern::Punct($crate::Punct::LArrow) };
    (<=)             => { $crate::TokenPattern::Punct($crate::Punct::Le) };
    (<)              => { $crate::TokenPattern::Punct($crate::Punct::Lt) };
    (-)              => { $crate::TokenPattern::Punct($crate::Punct::Minus) };
    (-=)             => { $crate::TokenPattern::Punct($crate::Punct::MinusEq) };
    (!=)             => { $crate::TokenPattern::Punct($crate::Punct::Ne) };
    (!)              => { $crate::TokenPattern::Punct($crate::Punct::Not) };
    (|)              => { $crate::TokenPattern::Punct($crate::Punct::Or) };
    (|=)             => { $crate::TokenPattern::Punct($crate::Punct::OrEq) };
    (||)             => { $crate::TokenPattern::Punct($crate::Punct::OrOr) };
    (::)             => { $crate::TokenPattern::Punct($crate::Punct::PathSep) };
    (%)              => { $crate::TokenPattern::Punct($crate::Punct::Percent) };
    (%=)             => { $crate::TokenPattern::Punct($crate::Punct::PercentEq) };
    (+)              => { $crate::TokenPattern::Punct($crate::Punct::Plus) };
    (+=)             => { $crate::TokenPattern::Punct($crate::Punct::PlusEq) };
    (#)              => { $crate::TokenPattern::Punct($crate::Punct::Pound) };
    (?)              => { $crate::TokenPattern::Punct($crate::Punct::Question) };
    (->)             => { $crate::TokenPattern::Punct($crate::Punct::RArrow) };
    (;)              => { $crate::TokenPattern::Punct($crate::Punct::Semi) };
    (<<)             => { $crate::TokenPattern::Punct($crate::Punct::Shl) };
    (<<=)            => { $crate::TokenPattern::Punct($crate::Punct::ShlEq) };
    (>>)             => { $crate::TokenPattern::Punct($crate::Punct::Shr) };
    (>>=)            => { $crate::TokenPattern::Punct($crate::Punct::ShrEq) };
    (/)              => { $crate::TokenPattern::Punct($crate::Punct::Slash) };
    (/=)             => { $crate::TokenPattern::Punct($crate::Punct::SlashEq) };
    (*)              => { $crate::TokenPattern::Punct($crate::Punct::Star) };
    (*=)             => { $crate::TokenPattern::Punct($crate::Punct::StarEq) };
    (~)              => { $crate::TokenPattern::Punct($crate::Punct::Tilde) };
    (_)              => { $crate::TokenPattern::Punct($crate::Punct::Underscore) };
    ($lit:literal)   => { $crate::TokenPattern::Literal($crate::LiteralPattern::parse(stringify!($lit)).unwrap()) };
    (true)           => { $crate::TokenPattern::Literal($crate::LiteralPattern::BoolLit::True) };
    (false)          => { $crate::TokenPattern::Literal($crate::LiteralPattern::BoolLit::False) };
    ($ident:ident)   => { $crate::TokenPattern::CustomKeyword($crate::Pattern::Specific($crate::InStr::from(stringify!($ident)))) };
	(!kw)		 	 => { $crate::TokenPattern::CustomKeyword($crate::Pattern::Wildcard) };
    (#$ident:ident)  => { $crate::TokenPattern::Ident($crate::Pattern::Specific($crate::InStr::from(stringify!($ident)))) };
	(!ident)		 => { $crate::TokenPattern::Ident($crate::Pattern::Wildcard) };
	(!float)		 => { $crate::TokenPattern::Literal($crate::LiteralPattern::FloatLit($crate::Pattern::Wildcard)) };
	(!bool)		 	 => { $crate::TokenPattern::Literal($crate::LiteralPattern::BoolLit($crate::Pattern::Wildcard)) };
	(!int)			 => { $crate::TokenPattern::Literal($crate::LiteralPattern::IntLit($crate::Pattern::Wildcard)) };
	(!char)			 => { $crate::TokenPattern::Literal($crate::LiteralPattern::CharLit($crate::Pattern::Wildcard)) };
	(!str)			 => { $crate::TokenPattern::Literal($crate::LiteralPattern::StringLit($crate::Pattern::Wildcard)) };
	(!byte)			 => { $crate::TokenPattern::Literal($crate::LiteralPattern::ByteLit($crate::Pattern::Wildcard)) };
	(!bytestr)		 => { $crate::TokenPattern::Literal($crate::LiteralPattern::ByteStringLit($crate::Pattern::Wildcard)) };
    (())             => { $crate::TokenPattern::Delimiter($crate::Delimiter::Paren) };
    ({})             => { $crate::TokenPattern::Delimiter($crate::Delimiter::Brace) };
    ([])             => { $crate::TokenPattern::Delimiter($crate::Delimiter::Bracket) };
}

#[test]
fn test_token_matches() {
    assert!(
        t![true].matches(TokenPattern::Literal(LiteralPattern::BoolLit(
            Pattern::Wildcard
        )))
    );
    assert!(t![false].matches(pat![!bool]));
    assert!(!t![struct].matches(pat![!bool]));
    assert!(!t![false].matches(pat![true]));
}
