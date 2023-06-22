extern crate alloc;

use crate::Literal;
use crate::Symbol;

use alloc::{format, string::String};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Token {
    Ident(Symbol),
    Literal(Literal),
    GroupPunct(GroupPunct),
    Punct(Punct),
    Keyword(Keyword),
    CustomKeyword(Symbol),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Keyword {
    Abstract,
    As,
    Async,
    Auto,
    Await,
    Become,
    Box,
    Break,
    Const,
    Continue,
    Crate,
    Default,
    Do,
    Dyn,
    Else,
    Enum,
    Extern,
    Final,
    Fn,
    For,
    If,
    Impl,
    In,
    Let,
    Loop,
    Macro,
    Match,
    Mod,
    Move,
    Mut,
    Override,
    Priv,
    Pub,
    Ref,
    Return,
    SelfType,
    SelfValue,
    Static,
    Struct,
    Super,
    Trait,
    Try,
    Type,
    Typeof,
    Union,
    Unsafe,
    Unsized,
    Use,
    Virtual,
    Where,
    While,
    Yield,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GroupPunct {
    Brace,
    Bracket,
    Paren,
}

impl GroupPunct {
    pub fn open(self) -> char {
        use GroupPunct::*;
        match self {
            Brace => '{',
            Bracket => '[',
            Paren => '(',
        }
    }

    pub fn close(self) -> char {
        use GroupPunct::*;
        match self {
            Brace => '}',
            Bracket => ']',
            Paren => ')',
        }
    }

    pub fn enclose<T: Into<String>>(self, enclosed: T) -> String {
        format!("{} {} {}", self.open(), enclosed.into(), self.close())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Punct {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Not,
    And,
    Or,
    AndAnd,
    OrOr,
    Shl,
    Shr,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    CaretEq,
    AndEq,
    OrEq,
    ShlEq,
    ShrEq,
    Eq,
    EqEq,
    Ne,
    Gt,
    Lt,
    Ge,
    LArrow,
    Le,
    At,
    Underscore,
    Dot,
    DotDot,
    DotDotDot,
    DotDotEq,
    Comma,
    Semi,
    Colon,
    PathSep,
    RArrow,
    FatArrow,
    Pound,
    Dollar,
    Question,
    Tilde,
}

impl From<Punct> for &'static str {
    fn from(value: Punct) -> Self {
        use Punct::*;
        match value {
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Percent => "%",
            Caret => "^",
            Not => "!",
            And => "&",
            Or => "|",
            AndAnd => "&&",
            OrOr => "||",
            Shl => "<<",
            Shr => ">>",
            PlusEq => "+=",
            MinusEq => "-=",
            StarEq => "*=",
            SlashEq => "/=",
            PercentEq => "%=",
            CaretEq => "^=",
            AndEq => "&=",
            OrEq => "|=",
            ShlEq => "<<=",
            ShrEq => ">>=",
            Eq => "=",
            EqEq => "==",
            Ne => "!=",
            Gt => ">",
            Lt => "<",
            Ge => ">=",
            LArrow => "<-",
            Le => "<=",
            At => "@",
            Underscore => "_",
            Dot => ".",
            DotDot => "..",
            DotDotDot => "...",
            DotDotEq => "..=",
            Comma => ",",
            Semi => ";",
            Colon => ":",
            PathSep => "::",
            RArrow => "->",
            FatArrow => "=>",
            Pound => "#",
            Dollar => "$",
            Question => "?",
            Tilde => "~",
        }
    }
}

impl From<&str> for Punct {
    fn from(value: &str) -> Self {
        use Punct::*;
        match value {
            "+" => Plus,
            "-" => Minus,
            "*" => Star,
            "/" => Slash,
            "%" => Percent,
            "^" => Caret,
            "!" => Not,
            "&" => And,
            "|" => Or,
            "&&" => AndAnd,
            "||" => OrOr,
            "<<" => Shl,
            ">>" => Shr,
            "+=" => PlusEq,
            "-=" => MinusEq,
            "*=" => StarEq,
            "/=" => SlashEq,
            "%=" => PercentEq,
            "^=" => CaretEq,
            "&=" => AndEq,
            "|=" => OrEq,
            "<<=" => ShlEq,
            ">>=" => ShrEq,
            "=" => Eq,
            "==" => EqEq,
            "!=" => Ne,
            ">" => Gt,
            "<" => Lt,
            ">=" => Ge,
            "<-" => LArrow,
            "<=" => Le,
            "@" => At,
            "_" => Underscore,
            "." => Dot,
            ".." => DotDot,
            "..." => DotDotDot,
            "..=" => DotDotEq,
            "," => Comma,
            ";" => Semi,
            ":" => Colon,
            "::" => PathSep,
            "->" => RArrow,
            "=>" => FatArrow,
            "#" => Pound,
            "$" => Dollar,
            "?" => Question,
            "~" => Tilde,
            _ => panic!("Invalid punctuation `{}`", value),
        }
    }
}

#[rustfmt::skip]
#[macro_export]
macro_rules! tt {
    (abstract)       => { $crate::Token::Keyword($crate::Keyword::Abstract) };
    (as)             => { $crate::Token::Keyword($crate::Keyword::As) };
    (async)          => { $crate::Token::Keyword($crate::Keyword::Async) };
    (auto)           => { $crate::Token::Keyword($crate::Keyword::Auto) };
    (await)          => { $crate::Token::Keyword($crate::Keyword::Await) };
    (become)         => { $crate::Token::Keyword($crate::Keyword::Become) };
    (box)            => { $crate::Token::Keyword($crate::Keyword::Box) };
    (break)          => { $crate::Token::Keyword($crate::Keyword::Break) };
    (const)          => { $crate::Token::Keyword($crate::Keyword::Const) };
    (continue)       => { $crate::Token::Keyword($crate::Keyword::Continue) };
    (crate)          => { $crate::Token::Keyword($crate::Keyword::Crate) };
    (default)        => { $crate::Token::Keyword($crate::Keyword::Default) };
    (do)             => { $crate::Token::Keyword($crate::Keyword::Do) };
    (dyn)            => { $crate::Token::Keyword($crate::Keyword::Dyn) };
    (else)           => { $crate::Token::Keyword($crate::Keyword::Else) };
    (enum)           => { $crate::Token::Keyword($crate::Keyword::Enum) };
    (extern)         => { $crate::Token::Keyword($crate::Keyword::Extern) };
    (final)          => { $crate::Token::Keyword($crate::Keyword::Final) };
    (fn)             => { $crate::Token::Keyword($crate::Keyword::Fn) };
    (for)            => { $crate::Token::Keyword($crate::Keyword::For) };
    (if)             => { $crate::Token::Keyword($crate::Keyword::If) };
    (impl)           => { $crate::Token::Keyword($crate::Keyword::Impl) };
    (in)             => { $crate::Token::Keyword($crate::Keyword::In) };
    (let)            => { $crate::Token::Keyword($crate::Keyword::Let) };
    (loop)           => { $crate::Token::Keyword($crate::Keyword::Loop) };
    (macro)          => { $crate::Token::Keyword($crate::Keyword::Macro) };
    (match)          => { $crate::Token::Keyword($crate::Keyword::Match) };
    (mod)            => { $crate::Token::Keyword($crate::Keyword::Mod) };
    (move)           => { $crate::Token::Keyword($crate::Keyword::Move) };
    (mut)            => { $crate::Token::Keyword($crate::Keyword::Mut) };
    (override)       => { $crate::Token::Keyword($crate::Keyword::Override) };
    (priv)           => { $crate::Token::Keyword($crate::Keyword::Priv) };
    (pub)            => { $crate::Token::Keyword($crate::Keyword::Pub) };
    (ref)            => { $crate::Token::Keyword($crate::Keyword::Ref) };
    (return)         => { $crate::Token::Keyword($crate::Keyword::Return) };
    (Self)           => { $crate::Token::Keyword($crate::Keyword::SelfType) };
    (self)           => { $crate::Token::Keyword($crate::Keyword::SelfValue) };
    (static)         => { $crate::Token::Keyword($crate::Keyword::Static) };
    (struct)         => { $crate::Token::Keyword($crate::Keyword::Struct) };
    (super)          => { $crate::Token::Keyword($crate::Keyword::Super) };
    (trait)          => { $crate::Token::Keyword($crate::Keyword::Trait) };
    (try)            => { $crate::Token::Keyword($crate::Keyword::Try) };
    (type)           => { $crate::Token::Keyword($crate::Keyword::Type) };
    (typeof)         => { $crate::Token::Keyword($crate::Keyword::Typeof) };
    (union)          => { $crate::Token::Keyword($crate::Keyword::Union) };
    (unsafe)         => { $crate::Token::Keyword($crate::Keyword::Unsafe) };
    (unsized)        => { $crate::Token::Keyword($crate::Keyword::Unsized) };
    (use)            => { $crate::Token::Keyword($crate::Keyword::Use) };
    (virtual)        => { $crate::Token::Keyword($crate::Keyword::Virtual) };
    (where)          => { $crate::Token::Keyword($crate::Keyword::Where) };
    (while)          => { $crate::Token::Keyword($crate::Keyword::While) };
    (yield)          => { $crate::Token::Keyword($crate::Keyword::Yield) };
    (&)              => { $crate::Token::Punct($crate::Punct::And) };
    (&&)             => { $crate::Token::Punct($crate::Punct::AndAnd) };
    (&=)             => { $crate::Token::Punct($crate::Punct::AndEq) };
    (@)              => { $crate::Token::Punct($crate::Punct::At) };
    (^)              => { $crate::Token::Punct($crate::Punct::Caret) };
    (^=)             => { $crate::Token::Punct($crate::Punct::CaretEq) };
    (:)              => { $crate::Token::Punct($crate::Punct::Colon) };
    (,)              => { $crate::Token::Punct($crate::Punct::Comma) };
    ($)              => { $crate::Token::Punct($crate::Punct::Dollar) };
    (.)              => { $crate::Token::Punct($crate::Punct::Dot) };
    (..)             => { $crate::Token::Punct($crate::Punct::DotDot) };
    (...)            => { $crate::Token::Punct($crate::Punct::DotDotDot) };
    (..=)            => { $crate::Token::Punct($crate::Punct::DotDotEq) };
    (=)              => { $crate::Token::Punct($crate::Punct::Eq) };
    (==)             => { $crate::Token::Punct($crate::Punct::EqEq) };
    (=>)             => { $crate::Token::Punct($crate::Punct::FatArrow) };
    (>=)             => { $crate::Token::Punct($crate::Punct::Ge) };
    (>)              => { $crate::Token::Punct($crate::Punct::Gt) };
    (<-)             => { $crate::Token::Punct($crate::Punct::LArrow) };
    (<=)             => { $crate::Token::Punct($crate::Punct::Le) };
    (<)              => { $crate::Token::Punct($crate::Punct::Lt) };
    (-)              => { $crate::Token::Punct($crate::Punct::Minus) };
    (-=)             => { $crate::Token::Punct($crate::Punct::MinusEq) };
    (!=)             => { $crate::Token::Punct($crate::Punct::Ne) };
    (!)              => { $crate::Token::Punct($crate::Punct::Not) };
    (|)              => { $crate::Token::Punct($crate::Punct::Or) };
    (|=)             => { $crate::Token::Punct($crate::Punct::OrEq) };
    (||)             => { $crate::Token::Punct($crate::Punct::OrOr) };
    (::)             => { $crate::Token::Punct($crate::Punct::PathSep) };
    (%)              => { $crate::Token::Punct($crate::Punct::Percent) };
    (%=)             => { $crate::Token::Punct($crate::Punct::PercentEq) };
    (+)              => { $crate::Token::Punct($crate::Punct::Plus) };
    (+=)             => { $crate::Token::Punct($crate::Punct::PlusEq) };
    (#)              => { $crate::Token::Punct($crate::Punct::Pound) };
    (?)              => { $crate::Token::Punct($crate::Punct::Question) };
    (->)             => { $crate::Token::Punct($crate::Punct::RArrow) };
    (;)              => { $crate::Token::Punct($crate::Punct::Semi) };
    (<<)             => { $crate::Token::Punct($crate::Punct::Shl) };
    (<<=)            => { $crate::Token::Punct($crate::Punct::ShlEq) };
    (>>)             => { $crate::Token::Punct($crate::Punct::Shr) };
    (>>=)            => { $crate::Token::Punct($crate::Punct::ShrEq) };
    (/)              => { $crate::Token::Punct($crate::Punct::Slash) };
    (/=)             => { $crate::Token::Punct($crate::Punct::SlashEq) };
    (*)              => { $crate::Token::Punct($crate::Punct::Star) };
    (*=)             => { $crate::Token::Punct($crate::Punct::StarEq) };
    (~)              => { $crate::Token::Punct($crate::Punct::Tilde) };
    (_)              => { $crate::Token::Punct($crate::Punct::Underscore) };
    ($lit:literal)   => { $crate::Literal::parse($lit).unwrap() };
    (true)           => { $crate::Literal::BoolLit::True };
    (false)          => { $crate::Literal::BoolLit::False };
    ($ident:ident)   => { $crate::Token::CustomKeyword($crate::Symbol::from(stringify!($ident))) };
    (#$ident:ident)  => { $crate::Token::Ident($crate::Symbol::from(stringify!($ident))) };
    (())             => { $crate::Token::GroupPunct($crate::GroupPunct::Paren) };
    ({})             => { $crate::Token::GroupPunct($crate::GroupPunct::Brace) };
    ([])             => { $crate::Token::GroupPunct($crate::GroupPunct::Bracket) };
}

#[macro_export]
macro_rules! assert_matches_sym {
    ($expr:expr, Token::$variant:ident($sym:literal)) => {
        assert!(matches!($expr, $crate::Token::$variant(_)));
        assert_eq!($expr, $crate::Token::$variant(Symbol::from($sym)));
    };
}

#[macro_export]
macro_rules! assert_matches_literal {
    ($expr:expr, Token::Literal(Literal::$variant:ident(sym:literal))) => {
        assert!(matches!(
            $expr,
            $crate::Token::Literal($crate::Token::Literal($crate::Literal::$variant(_)))
        ));
        assert_eq!(
            $expr,
            $crate::Token::Literal($crate::Literal::$variant::parse(Symbol::from($sym)))
        );
    };
}
