#[derive(Clone, PartialEq, Hash)]
pub enum Token {
    Ident(String),
    Literal(String),
    GroupPunct(GroupPunct),
    Punct(Punct),
}

#[derive(Clone, Copy, PartialEq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Hash)]
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
    RArrrow,
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
            RArrrow => "->",
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
            "->" => RArrrow,
            "=>" => FatArrow,
            "#" => Pound,
            "$" => Dollar,
            "?" => Question,
            "~" => Tilde,
            _ => panic!("Invalid punctuation `{}`", value),
        }
    }
}
