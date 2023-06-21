use sin::*;
use sin_macros::test_backend;

#[test]
fn test_enclose() {
    assert_eq!(GroupPunct::Brace.enclose("test"), "{ test }");
    assert_eq!(GroupPunct::Bracket.enclose("test"), "[ test ]");
    assert_eq!(GroupPunct::Paren.enclose("test"), "( test )");
}

#[test]
fn test_tt_punct() {
    assert!(matches!(tt![,], Token::Punct(_)));
    assert!(matches!(tt![,], Token::Punct(Punct::Comma)));
    assert!(matches!(tt![#], Token::Punct(Punct::Pound)));
    assert!(matches!(tt![;], Token::Punct(Punct::Semi)));
    assert!(matches!(tt![{}], Token::GroupPunct(GroupPunct::Brace)));
    assert!(matches!(tt![>=], Token::Punct(Punct::Ge)));

    assert!(matches!(tt![&], Token::Punct(Punct::And)));
    assert!(matches!(tt![&&], Token::Punct(Punct::AndAnd)));
    assert!(matches!(tt![&=], Token::Punct(Punct::AndEq)));
    assert!(matches!(tt![@], Token::Punct(Punct::At)));
    assert!(matches!(tt![^], Token::Punct(Punct::Caret)));
    assert!(matches!(tt![^=], Token::Punct(Punct::CaretEq)));
    assert!(matches!(tt![:], Token::Punct(Punct::Colon)));
    assert!(matches!(tt![,], Token::Punct(Punct::Comma)));
    assert!(matches!(tt![$], Token::Punct(Punct::Dollar)));
    assert!(matches!(tt![.], Token::Punct(Punct::Dot)));
    assert!(matches!(tt![..], Token::Punct(Punct::DotDot)));
    assert!(matches!(tt![...], Token::Punct(Punct::DotDotDot)));
    assert!(matches!(tt![..=], Token::Punct(Punct::DotDotEq)));
    assert!(matches!(tt![=], Token::Punct(Punct::Eq)));
    assert!(matches!(tt![==], Token::Punct(Punct::EqEq)));
    assert!(matches!(tt![=>], Token::Punct(Punct::FatArrow)));
    assert!(matches!(tt![>=], Token::Punct(Punct::Ge)));
    assert!(matches!(tt![>], Token::Punct(Punct::Gt)));
    assert!(matches!(tt![<-], Token::Punct(Punct::LArrow)));
    assert!(matches!(tt![<=], Token::Punct(Punct::Le)));
    assert!(matches!(tt![<], Token::Punct(Punct::Lt)));
    assert!(matches!(tt![-], Token::Punct(Punct::Minus)));
    assert!(matches!(tt![-=], Token::Punct(Punct::MinusEq)));
    assert!(matches!(tt![!=], Token::Punct(Punct::Ne)));
    assert!(matches!(tt![!], Token::Punct(Punct::Not)));
    assert!(matches!(tt![|], Token::Punct(Punct::Or)));
    assert!(matches!(tt![|=], Token::Punct(Punct::OrEq)));
    assert!(matches!(tt![||], Token::Punct(Punct::OrOr)));
    assert!(matches!(tt![::], Token::Punct(Punct::PathSep)));
    assert!(matches!(tt![%], Token::Punct(Punct::Percent)));
    assert!(matches!(tt![%=], Token::Punct(Punct::PercentEq)));
    assert!(matches!(tt![+], Token::Punct(Punct::Plus)));
    assert!(matches!(tt![+=], Token::Punct(Punct::PlusEq)));
    assert!(matches!(tt![#], Token::Punct(Punct::Pound)));
    assert!(matches!(tt![?], Token::Punct(Punct::Question)));
    assert!(matches!(tt![->], Token::Punct(Punct::RArrow)));
    assert!(matches!(tt![;], Token::Punct(Punct::Semi)));
    assert!(matches!(tt![<<], Token::Punct(Punct::Shl)));
    assert!(matches!(tt![<<=], Token::Punct(Punct::ShlEq)));
    assert!(matches!(tt![>>], Token::Punct(Punct::Shr)));
    assert!(matches!(tt![>>=], Token::Punct(Punct::ShrEq)));
    assert!(matches!(tt![/], Token::Punct(Punct::Slash)));
    assert!(matches!(tt![/=], Token::Punct(Punct::SlashEq)));
    assert!(matches!(tt![*], Token::Punct(Punct::Star)));
    assert!(matches!(tt![*=], Token::Punct(Punct::StarEq)));
    assert!(matches!(tt![~], Token::Punct(Punct::Tilde)));
    assert!(matches!(tt![_], Token::Punct(Punct::Underscore)));
}

#[test]
fn test_tt_custom_keywords() {
    use Token::CustomKeyword;
    assert!(matches!(tt![something], CustomKeyword("something")));
    assert!(matches!(tt![AnotherThing], CustomKeyword("AnotherThing")));
}

#[test]
fn test_tt_keywords() {
    assert!(matches!(tt![struct], Token::Keyword(Keyword::Struct)));
    assert!(matches!(tt![abstract], Token::Keyword(Keyword::Abstract)));
    assert!(matches!(tt![as], Token::Keyword(Keyword::As)));
    assert!(matches!(tt![async], Token::Keyword(Keyword::Async)));
    assert!(matches!(tt![auto], Token::Keyword(Keyword::Auto)));
    assert!(matches!(tt![await], Token::Keyword(Keyword::Await)));
    assert!(matches!(tt![become], Token::Keyword(Keyword::Become)));
    assert!(matches!(tt![box], Token::Keyword(Keyword::Box)));
    assert!(matches!(tt![break], Token::Keyword(Keyword::Break)));
    assert!(matches!(tt![const], Token::Keyword(Keyword::Const)));
    assert!(matches!(tt![continue], Token::Keyword(Keyword::Continue)));
    assert!(matches!(tt![crate], Token::Keyword(Keyword::Crate)));
    assert!(matches!(tt![default], Token::Keyword(Keyword::Default)));
    assert!(matches!(tt![do], Token::Keyword(Keyword::Do)));
    assert!(matches!(tt![dyn], Token::Keyword(Keyword::Dyn)));
    assert!(matches!(tt![else], Token::Keyword(Keyword::Else)));
    assert!(matches!(tt![enum], Token::Keyword(Keyword::Enum)));
    assert!(matches!(tt![extern], Token::Keyword(Keyword::Extern)));
    assert!(matches!(tt![final], Token::Keyword(Keyword::Final)));
    assert!(matches!(tt![fn], Token::Keyword(Keyword::Fn)));
    assert!(matches!(tt![for], Token::Keyword(Keyword::For)));
    assert!(matches!(tt![if], Token::Keyword(Keyword::If)));
    assert!(matches!(tt![impl], Token::Keyword(Keyword::Impl)));
    assert!(matches!(tt![in], Token::Keyword(Keyword::In)));
    assert!(matches!(tt![let], Token::Keyword(Keyword::Let)));
    assert!(matches!(tt![loop], Token::Keyword(Keyword::Loop)));
    assert!(matches!(tt![macro], Token::Keyword(Keyword::Macro)));
    assert!(matches!(tt![match], Token::Keyword(Keyword::Match)));
    assert!(matches!(tt![mod], Token::Keyword(Keyword::Mod)));
    assert!(matches!(tt![move], Token::Keyword(Keyword::Move)));
    assert!(matches!(tt![mut], Token::Keyword(Keyword::Mut)));
    assert!(matches!(tt![override], Token::Keyword(Keyword::Override)));
    assert!(matches!(tt![priv], Token::Keyword(Keyword::Priv)));
    assert!(matches!(tt![pub], Token::Keyword(Keyword::Pub)));
    assert!(matches!(tt![ref], Token::Keyword(Keyword::Ref)));
    assert!(matches!(tt![return], Token::Keyword(Keyword::Return)));
    assert!(matches!(tt![Self], Token::Keyword(Keyword::SelfType)));
    assert!(matches!(tt![self], Token::Keyword(Keyword::SelfValue)));
    assert!(matches!(tt![static], Token::Keyword(Keyword::Static)));
    assert!(matches!(tt![struct], Token::Keyword(Keyword::Struct)));
    assert!(matches!(tt![super], Token::Keyword(Keyword::Super)));
    assert!(matches!(tt![trait], Token::Keyword(Keyword::Trait)));
    assert!(matches!(tt![try], Token::Keyword(Keyword::Try)));
    assert!(matches!(tt![type], Token::Keyword(Keyword::Type)));
    assert!(matches!(tt![typeof], Token::Keyword(Keyword::Typeof)));
    assert!(matches!(tt![union], Token::Keyword(Keyword::Union)));
    assert!(matches!(tt![unsafe], Token::Keyword(Keyword::Unsafe)));
    assert!(matches!(tt![unsized], Token::Keyword(Keyword::Unsized)));
    assert!(matches!(tt![use], Token::Keyword(Keyword::Use)));
    assert!(matches!(tt![virtual], Token::Keyword(Keyword::Virtual)));
    assert!(matches!(tt![where], Token::Keyword(Keyword::Where)));
    assert!(matches!(tt![while], Token::Keyword(Keyword::While)));
    assert!(matches!(tt![yield], Token::Keyword(Keyword::Yield)));
}

#[test]
fn test_tt_ident() {
    assert!(matches!(tt![#ident], Token::Ident("ident")));
    assert!(matches!(tt![#some_thing], Token::Ident("some_thing")));
    assert!(matches!(tt![#SomeThing], Token::Ident("SomeThing")));
    assert!(!matches!(tt![struct], Token::Ident(_)));
    assert!(!matches!(tt![something], Token::Ident(_)));
}

#[test]
fn backend() {
    test_backend!(theasdfasdfse are tokens);
}
