use crate::{Literal, Symbol};
use core::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenParseError {
    pub msg: Symbol,
}

impl From<&str> for TokenParseError {
    fn from(value: &str) -> Self {
        TokenParseError {
            msg: Symbol::from(value),
        }
    }
}

impl From<Symbol> for TokenParseError {
    fn from(value: Symbol) -> Self {
        TokenParseError { msg: value }
    }
}

impl From<&Symbol> for TokenParseError {
    fn from(value: &Symbol) -> Self {
        TokenParseError { msg: *value }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Token {
    Ident(Symbol),
    Literal(Literal),
    GroupPunct(GroupPunct),
    Punct(Punct),
    Keyword(Keyword),
    CustomKeyword(Symbol),
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

// impl Keyword {
//     pub fn as_str(&self)
// }

impl TryFrom<&str> for Keyword {
    type Error = TokenParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Keyword::*;
        match value {
            "abstract" => Ok(Abstract),
            "as" => Ok(As),
            "async" => Ok(Async),
            "auto" => Ok(Auto),
            "await" => Ok(Await),
            "become" => Ok(Become),
            "box" => Ok(Box),
            "break" => Ok(Break),
            "const" => Ok(Const),
            "continue" => Ok(Continue),
            "crate" => Ok(Crate),
            "default" => Ok(Default),
            "do" => Ok(Do),
            "dyn" => Ok(Dyn),
            "else" => Ok(Else),
            "enum" => Ok(Enum),
            "extern" => Ok(Extern),
            "final" => Ok(Final),
            "fn" => Ok(Fn),
            "for" => Ok(For),
            "if" => Ok(If),
            "impl" => Ok(Impl),
            "in" => Ok(In),
            "let" => Ok(Let),
            "loop" => Ok(Loop),
            "macro" => Ok(Macro),
            "match" => Ok(Match),
            "mod" => Ok(Mod),
            "move" => Ok(Move),
            "mut" => Ok(Mut),
            "override" => Ok(Override),
            "priv" => Ok(Priv),
            "pub" => Ok(Pub),
            "ref" => Ok(Ref),
            "return" => Ok(Return),
            "Self" => Ok(SelfType),
            "self" => Ok(SelfValue),
            "static" => Ok(Static),
            "struct" => Ok(Struct),
            "super" => Ok(Super),
            "trait" => Ok(Trait),
            "try" => Ok(Try),
            "type" => Ok(Type),
            "typeof" => Ok(Typeof),
            "union" => Ok(Union),
            "unsafe" => Ok(Unsafe),
            "unsized" => Ok(Unsized),
            "use" => Ok(Use),
            "virtual" => Ok(Virtual),
            "where" => Ok(Where),
            "while" => Ok(While),
            "yield" => Ok(Yield),
            _ => Err(TokenParseError::from("Invalid keyword '{value}'")),
        }
    }
}

impl From<Keyword> for &'static str {
    fn from(value: Keyword) -> Self {
        use Keyword::*;
        match value {
            Abstract => "abstract",
            As => "as",
            Async => "async",
            Auto => "auto",
            Await => "await",
            Become => "become",
            Box => "box",
            Break => "break",
            Const => "const",
            Continue => "continue",
            Crate => "crate",
            Default => "default",
            Do => "do",
            Dyn => "dyn",
            Else => "else",
            Enum => "enum",
            Extern => "extern",
            Final => "final",
            Fn => "fn",
            For => "for",
            If => "if",
            Impl => "impl",
            In => "in",
            Let => "let",
            Loop => "loop",
            Macro => "macro",
            Match => "match",
            Mod => "mod",
            Move => "move",
            Mut => "mut",
            Override => "override",
            Priv => "priv",
            Pub => "pub",
            Ref => "ref",
            Return => "return",
            SelfType => "Self",
            SelfValue => "self",
            Static => "static",
            Struct => "struct",
            Super => "super",
            Trait => "trait",
            Try => "try",
            Type => "type",
            Typeof => "typeof",
            Union => "union",
            Unsafe => "unsafe",
            Unsized => "unsized",
            Use => "use",
            Virtual => "virtual",
            Where => "where",
            While => "while",
            Yield => "yield",
        }
    }
}

impl From<&Keyword> for &'static str {
    fn from(value: &Keyword) -> Self {
        (*value).into()
    }
}

impl From<Keyword> for Symbol {
    fn from(value: Keyword) -> Self {
        let value: &'static str = value.into();
        Symbol::from(value)
    }
}

impl From<&Keyword> for Symbol {
    fn from(value: &Keyword) -> Self {
        let value: &'static str = value.into();
        Symbol::from(value)
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str((*self).into())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GroupPunct {
    Brace,
    Bracket,
    Paren,
}

// TODO: add struct Group(GroupPunct, TokenStream) that contains this

impl GroupPunct {
    pub const fn open(self) -> char {
        use GroupPunct::*;
        match self {
            Brace => '{',
            Bracket => '[',
            Paren => '(',
        }
    }

    pub const fn close(self) -> char {
        use GroupPunct::*;
        match self {
            Brace => '}',
            Bracket => ']',
            Paren => ')',
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

impl TryFrom<&str> for Punct {
    type Error = TokenParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Punct::*;
        match value {
            "+" => Ok(Plus),
            "-" => Ok(Minus),
            "*" => Ok(Star),
            "/" => Ok(Slash),
            "%" => Ok(Percent),
            "^" => Ok(Caret),
            "!" => Ok(Not),
            "&" => Ok(And),
            "|" => Ok(Or),
            "&&" => Ok(AndAnd),
            "||" => Ok(OrOr),
            "<<" => Ok(Shl),
            ">>" => Ok(Shr),
            "+=" => Ok(PlusEq),
            "-=" => Ok(MinusEq),
            "*=" => Ok(StarEq),
            "/=" => Ok(SlashEq),
            "%=" => Ok(PercentEq),
            "^=" => Ok(CaretEq),
            "&=" => Ok(AndEq),
            "|=" => Ok(OrEq),
            "<<=" => Ok(ShlEq),
            ">>=" => Ok(ShrEq),
            "=" => Ok(Eq),
            "==" => Ok(EqEq),
            "!=" => Ok(Ne),
            ">" => Ok(Gt),
            "<" => Ok(Lt),
            ">=" => Ok(Ge),
            "<-" => Ok(LArrow),
            "<=" => Ok(Le),
            "@" => Ok(At),
            "_" => Ok(Underscore),
            "." => Ok(Dot),
            ".." => Ok(DotDot),
            "..." => Ok(DotDotDot),
            "..=" => Ok(DotDotEq),
            "," => Ok(Comma),
            ";" => Ok(Semi),
            ":" => Ok(Colon),
            "::" => Ok(PathSep),
            "->" => Ok(RArrow),
            "=>" => Ok(FatArrow),
            "#" => Ok(Pound),
            "$" => Ok(Dollar),
            "?" => Ok(Question),
            "~" => Ok(Tilde),
            _ => Err(TokenParseError::from("Invalid punctuation `{value}`")),
        }
    }
}

impl From<&Punct> for &'static str {
    fn from(value: &Punct) -> Self {
        (*value).into()
    }
}

impl From<Punct> for Symbol {
    fn from(value: Punct) -> Self {
        let value: &'static str = value.into();
        Symbol::from(value)
    }
}

impl From<&Punct> for Symbol {
    fn from(value: &Punct) -> Self {
        let value: &'static str = value.into();
        Symbol::from(value)
    }
}

impl Display for Punct {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.into())
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
    ($lit:literal)   => { $crate::Token::Literal($crate::Literal::parse(stringify!($lit)).unwrap()) };
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

#[test]
fn test_token_traits() {
    use crate::util::*;
    assert_golden_traits::<Token>();
}
