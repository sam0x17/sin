use std::fmt::Display;

use interned::Interned;

use crate::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[must_use]
pub enum Pattern<T> {
    Specific(T),
    Wildcard,
}

pub trait PatternName {
    fn pattern_name(&self) -> &'static str;
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
    Nothing,
    Wildcard,
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
            (_, TokenPattern::Wildcard) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum LiteralPattern {
    Bool(Pattern<bool>),
    Char(Pattern<char>),
    Integer(Pattern<IntLit>),
    Float(Pattern<FloatLit>),
    String(Pattern<InStr>),
    Byte(Pattern<ByteLit>),
    ByteString(Pattern<ByteStringLit>),
    /// Matches any valid literal
    Wildcard,
}

impl ParseLiteral for LiteralPattern {
    fn parse<S: AsRef<str>>(input: S) -> Result<Self, litrs::ParseError> {
        let sym = InStr::from(input.as_ref());
        match litrs::Literal::parse(sym.as_str()) {
            Ok(litrs::Literal::Bool(lit)) => {
                Ok(LiteralPattern::Bool(Pattern::Specific(lit.value())))
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
            (Literal::Bool(b), LiteralPattern::Bool(pat)) => b.matches(pat),
            (Literal::Char(c), LiteralPattern::Char(pat)) => c.matches(pat),
            (Literal::Integer(i), LiteralPattern::Integer(pat)) => i.matches(pat),
            (Literal::Float(f), LiteralPattern::Float(pat)) => f.matches(pat),
            (Literal::String(s), LiteralPattern::String(pat)) => s.matches(pat),
            (Literal::Byte(b), LiteralPattern::Byte(pat)) => b.matches(pat),
            (Literal::ByteString(b), LiteralPattern::ByteString(pat)) => b.matches(pat),
            (_, LiteralPattern::Wildcard) => true,
            _ => false,
        }
    }
}

impl From<Literal> for LiteralPattern {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::Bool(b) => LiteralPattern::Bool(Specific(b)),
            Literal::Char(c) => LiteralPattern::Char(Specific(c)),
            Literal::Integer(i) => LiteralPattern::Integer(Specific(i)),
            Literal::Float(f) => LiteralPattern::Float(Specific(f)),
            Literal::String(s) => LiteralPattern::String(Specific(s)),
            Literal::Byte(b) => LiteralPattern::Byte(Specific(b)),
            Literal::ByteString(bs) => LiteralPattern::ByteString(Specific(bs)),
        }
    }
}

impl From<Token> for TokenPattern {
    fn from(token: Token) -> Self {
        match token {
            Token::Ident(ident) => TokenPattern::Ident(Pattern::Specific(ident)),
            Token::Literal(lit) => TokenPattern::Literal(lit.into()),
            Token::Delimiter(delim) => TokenPattern::Delimiter(Specific(delim)),
            Token::Punct(punct) => TokenPattern::Punct(Specific(punct)),
            Token::Keyword(kw) => TokenPattern::Keyword(Specific(kw)),
            Token::CustomKeyword(ckw) => TokenPattern::CustomKeyword(Specific(ckw)),
        }
    }
}

impl Display for LiteralPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralPattern::Bool(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("bool literal"),
            },
            LiteralPattern::Char(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("char literal"),
            },
            LiteralPattern::Integer(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("integer literal"),
            },
            LiteralPattern::Float(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("float literal"),
            },
            LiteralPattern::String(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("string literal"),
            },
            LiteralPattern::Byte(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("byte literal"),
            },
            LiteralPattern::ByteString(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("byte string literal"),
            },
            LiteralPattern::Wildcard => f.write_str("literal"),
        }
    }
}

impl Display for TokenPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenPattern::Ident(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("ident"),
            },
            TokenPattern::Literal(val) => val.fmt(f),
            TokenPattern::Delimiter(val) => match val {
                Specific(val) => match val {
                    Delimiter::Brace => f.write_str("brace"),
                    Delimiter::Bracket => f.write_str("bracket"),
                    Delimiter::Paren => f.write_str("parenthesis"),
                },
                Wildcard => f.write_str("delimiter"),
            },
            TokenPattern::Punct(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("punctuation"),
            },
            TokenPattern::Keyword(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("keyword"),
            },
            TokenPattern::CustomKeyword(val) => match val {
                Specific(val) => f.write_fmt(format_args!("`{val}`")),
                Wildcard => f.write_str("custom keyword"),
            },
            TokenPattern::Nothing => f.write_str("nothing"),
            TokenPattern::Wildcard => f.write_str("token"),
        }
    }
}

#[rustfmt::skip]
#[macro_export]
macro_rules! pat {
    ()               => { $crate::TokenPattern::Nothing };
    (abstract)       => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Abstract)) };
    (as)             => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::As)) };
    (async)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Async)) };
    (auto)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Auto)) };
    (await)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Await)) };
    (become)         => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Become)) };
    (box)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Box)) };
    (break)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Break)) };
    (const)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Const)) };
    (continue)       => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Continue)) };
    (crate)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Crate)) };
    (default)        => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Default)) };
    (do)             => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Do)) };
    (dyn)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Dyn)) };
    (else)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Else)) };
    (enum)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Enum)) };
    (extern)         => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Extern)) };
    (final)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Final)) };
    (fn)             => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Fn)) };
    (for)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::For)) };
    (if)             => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::If)) };
    (impl)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Impl)) };
    (in)             => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::In)) };
    (let)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Let)) };
    (loop)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Loop)) };
    (macro)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Macro)) };
    (match)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Match)) };
    (mod)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Mod)) };
    (move)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Move)) };
    (mut)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Mut)) };
    (override)       => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Override)) };
    (priv)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Priv)) };
    (pub)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Pub)) };
    (ref)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Ref)) };
    (return)         => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Return)) };
    (Self)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::SelfType)) };
    (self)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::SelfValue)) };
    (static)         => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Static)) };
    (struct)         => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Struct)) };
    (super)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Super)) };
    (trait)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Trait)) };
    (try)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Try)) };
    (type)           => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Type)) };
    (typeof)         => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Typeof)) };
    (union)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Union)) };
    (unsafe)         => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Unsafe)) };
    (unsized)        => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Unsized)) };
    (use)            => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Use)) };
    (virtual)        => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Virtual)) };
    (where)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Where)) };
    (while)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::While)) };
    (yield)          => { $crate::TokenPattern::Keyword($crate::Pattern::Specific($crate::Keyword::Yield)) };
    (&)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::And)) };
    (&&)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::AndAnd)) };
    (&=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::AndEq)) };
    (@)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::At)) };
    (^)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Caret)) };
    (^=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::CaretEq)) };
    (:)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Colon)) };
    (,)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Comma)) };
    ($)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Dollar)) };
    (.)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Dot)) };
    (..)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::DotDot)) };
    (...)            => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::DotDotDot)) };
    (..=)            => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::DotDotEq)) };
    (=)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Eq)) };
    (==)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::EqEq)) };
    (=>)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::FatArrow)) };
    (>=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Ge)) };
    (>)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Gt)) };
    (<-)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::LArrow)) };
    (<=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Le)) };
    (<)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Lt)) };
    (-)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Minus)) };
    (-=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::MinusEq)) };
    (!=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Ne)) };
    (!)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Not)) };
    (|)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Or)) };
    (|=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::OrEq)) };
    (||)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::OrOr)) };
    (::)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::PathSep)) };
    (%)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Percent)) };
    (%=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::PercentEq)) };
    (+)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Plus)) };
    (+=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::PlusEq)) };
    (#)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Pound)) };
    (?)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Question)) };
    (->)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::RArrow)) };
    (;)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Semi)) };
    (<<)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Shl)) };
    (<<=)            => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::ShlEq)) };
    (>>)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Shr)) };
    (>>=)            => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::ShrEq)) };
    (/)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Slash)) };
    (/=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::SlashEq)) };
    (*)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Star)) };
    (*=)             => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::StarEq)) };
    (~)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Tilde)) };
    (_)              => { $crate::TokenPattern::Punct($crate::Pattern::Specific($crate::Punct::Underscore)) };
    ($lit:literal)   => { $crate::TokenPattern::Literal($crate::LiteralPattern::parse(stringify!($lit)).unwrap()) };
    (true)           => { $crate::TokenPattern::Literal($crate::LiteralPattern::BoolLit::True) };
    (false)          => { $crate::TokenPattern::Literal($crate::LiteralPattern::BoolLit::False) };
    (!lit)           => { $crate::TokenPattern::Literal($crate::LiteralPattern::Wildcard) };
    (!kw)            => { $crate::TokenPattern::Keyword($crate::Pattern::Wildcard) };
    ($ident:ident)   => { $crate::TokenPattern::CustomKeyword($crate::Pattern::Specific($crate::InStr::from(stringify!($ident)))) };
	(!ckw)		 	 => { $crate::TokenPattern::CustomKeyword($crate::Pattern::Wildcard) };
    (#$ident:ident)  => { $crate::TokenPattern::Ident($crate::Pattern::Specific($crate::InStr::from(stringify!($ident)))) };
	(!ident)		 => { $crate::TokenPattern::Ident($crate::Pattern::Wildcard) };
	(!float)		 => { $crate::TokenPattern::Literal($crate::LiteralPattern::Float($crate::Pattern::Wildcard)) };
	(!bool)		 	 => { $crate::TokenPattern::Literal($crate::LiteralPattern::Bool($crate::Pattern::Wildcard)) };
	(!int)			 => { $crate::TokenPattern::Literal($crate::LiteralPattern::Integer($crate::Pattern::Wildcard)) };
	(!char)			 => { $crate::TokenPattern::Literal($crate::LiteralPattern::Char($crate::Pattern::Wildcard)) };
	(!str)			 => { $crate::TokenPattern::Literal($crate::LiteralPattern::String($crate::Pattern::Wildcard)) };
	(!byte)			 => { $crate::TokenPattern::Literal($crate::LiteralPattern::Byte($crate::Pattern::Wildcard)) };
	(!bytestr)		 => { $crate::TokenPattern::Literal($crate::LiteralPattern::ByteString($crate::Pattern::Wildcard)) };
    (!punct)         => { $crate::TokenPattern::Punct($crate::Pattern::Wildcard) };
    (!delim)         => { $crate::TokenPattern::Delimiter($crate::Pattern::Wildcard) };
    (!token)         => { $crate::TokenPattern::Wildcard };
    (())             => { $crate::TokenPattern::Delimiter($crate::Pattern::Specific($crate::Delimiter::Paren)) };
    ({})             => { $crate::TokenPattern::Delimiter($crate::Pattern::Specific($crate::Delimiter::Brace)) };
    ([])             => { $crate::TokenPattern::Delimiter($crate::Pattern::Specific($crate::Delimiter::Bracket)) };
}

#[test]
fn test_token_matches() {
    assert!(t![true].matches(TokenPattern::Literal(LiteralPattern::Bool(
        Pattern::Wildcard
    ))));
    assert!(t![false].matches(pat![!bool]));
    assert!(!t![struct].matches(pat![!bool]));
    assert!(!t![false].matches(pat![true]));
    assert!(t![static].matches(pat![static]));
    assert!(t![()].matches(pat![()]));
    assert!(!t![{}].matches(pat![[]]));
    assert!(t![custom_keyword].matches(pat![custom_keyword]));
    assert!(t![#my_ident].matches(pat![#my_ident]));
    assert!(t![#my_other_ident].matches(pat![!ident]));
    assert!(!t![some_keyword].matches(pat![!ident]));
    assert!(t![some_keyword].matches(pat![!ckw]));
    assert!(!t![some_keyword].matches(pat![!kw]));
    assert!(t![override].matches(pat![!kw]));
    assert!(t![33.75].matches(pat![!float]));
    assert!(t![535].matches(pat![535]));
    assert!(t![300_000_000].matches(pat![!int]));
    assert!(t!["hey"].matches(pat![!lit]));
    assert!(t![343894].matches(pat![!lit]));
    assert!(t![false].matches(pat![!lit]));
    assert!(!t![something].matches(TokenPattern::Nothing));
    assert!(t![false].matches(TokenPattern::Wildcard));
}
