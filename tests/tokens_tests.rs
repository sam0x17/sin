use sin::*;

#[test]
fn test_tt_group_punct() {
    assert!(matches!(t![{}], Token::Delimiter(Delimiter::Brace)));
    assert!(matches!(t![()], Token::Delimiter(Delimiter::Paren)));
    assert!(matches!(t![[]], Token::Delimiter(Delimiter::Bracket)));
}

#[test]
fn test_tt_punct() {
    assert!(matches!(t![,], Token::Punct(_)));
    assert!(matches!(t![,], Token::Punct(Punct::Comma)));
    assert!(matches!(t![#], Token::Punct(Punct::Pound)));
    assert!(matches!(t![;], Token::Punct(Punct::Semi)));
    assert!(matches!(t![>=], Token::Punct(Punct::Ge)));
    assert!(matches!(t![&], Token::Punct(Punct::And)));
    assert!(matches!(t![&&], Token::Punct(Punct::AndAnd)));
    assert!(matches!(t![&=], Token::Punct(Punct::AndEq)));
    assert!(matches!(t![@], Token::Punct(Punct::At)));
    assert!(matches!(t![^], Token::Punct(Punct::Caret)));
    assert!(matches!(t![^=], Token::Punct(Punct::CaretEq)));
    assert!(matches!(t![:], Token::Punct(Punct::Colon)));
    assert!(matches!(t![,], Token::Punct(Punct::Comma)));
    assert!(matches!(t![$], Token::Punct(Punct::Dollar)));
    assert!(matches!(t![.], Token::Punct(Punct::Dot)));
    assert!(matches!(t![..], Token::Punct(Punct::DotDot)));
    assert!(matches!(t![...], Token::Punct(Punct::DotDotDot)));
    assert!(matches!(t![..=], Token::Punct(Punct::DotDotEq)));
    assert!(matches!(t![=], Token::Punct(Punct::Eq)));
    assert!(matches!(t![==], Token::Punct(Punct::EqEq)));
    assert!(matches!(t![=>], Token::Punct(Punct::FatArrow)));
    assert!(matches!(t![>=], Token::Punct(Punct::Ge)));
    assert!(matches!(t![>], Token::Punct(Punct::Gt)));
    assert!(matches!(t![<-], Token::Punct(Punct::LArrow)));
    assert!(matches!(t![<=], Token::Punct(Punct::Le)));
    assert!(matches!(t![<], Token::Punct(Punct::Lt)));
    assert!(matches!(t![-], Token::Punct(Punct::Minus)));
    assert!(matches!(t![-=], Token::Punct(Punct::MinusEq)));
    assert!(matches!(t![!=], Token::Punct(Punct::Ne)));
    assert!(matches!(t![!], Token::Punct(Punct::Not)));
    assert!(matches!(t![|], Token::Punct(Punct::Or)));
    assert!(matches!(t![|=], Token::Punct(Punct::OrEq)));
    assert!(matches!(t![||], Token::Punct(Punct::OrOr)));
    assert!(matches!(t![::], Token::Punct(Punct::PathSep)));
    assert!(matches!(t![%], Token::Punct(Punct::Percent)));
    assert!(matches!(t![%=], Token::Punct(Punct::PercentEq)));
    assert!(matches!(t![+], Token::Punct(Punct::Plus)));
    assert!(matches!(t![+=], Token::Punct(Punct::PlusEq)));
    assert!(matches!(t![#], Token::Punct(Punct::Pound)));
    assert!(matches!(t![?], Token::Punct(Punct::Question)));
    assert!(matches!(t![->], Token::Punct(Punct::RArrow)));
    assert!(matches!(t![;], Token::Punct(Punct::Semi)));
    assert!(matches!(t![<<], Token::Punct(Punct::Shl)));
    assert!(matches!(t![<<=], Token::Punct(Punct::ShlEq)));
    assert!(matches!(t![>>], Token::Punct(Punct::Shr)));
    assert!(matches!(t![>>=], Token::Punct(Punct::ShrEq)));
    assert!(matches!(t![/], Token::Punct(Punct::Slash)));
    assert!(matches!(t![/=], Token::Punct(Punct::SlashEq)));
    assert!(matches!(t![*], Token::Punct(Punct::Star)));
    assert!(matches!(t![*=], Token::Punct(Punct::StarEq)));
    assert!(matches!(t![~], Token::Punct(Punct::Tilde)));
    assert!(matches!(t![_], Token::Punct(Punct::Underscore)));
}

#[test]
fn test_tt_custom_keywords() {
    assert_matches_sym!(t![something], Token::CustomKeyword("something"));
    assert_matches_sym!(t![AnotherThing], Token::CustomKeyword("AnotherThing"));
    assert_matches_sym!(t![more_things], Token::CustomKeyword("more_things"));
}

#[test]
fn test_tt_keywords() {
    assert!(matches!(t![struct], Token::Keyword(Keyword::Struct)));
    assert!(matches!(t![abstract], Token::Keyword(Keyword::Abstract)));
    assert!(matches!(t![as], Token::Keyword(Keyword::As)));
    assert!(matches!(t![async], Token::Keyword(Keyword::Async)));
    assert!(matches!(t![auto], Token::Keyword(Keyword::Auto)));
    assert!(matches!(t![await], Token::Keyword(Keyword::Await)));
    assert!(matches!(t![become], Token::Keyword(Keyword::Become)));
    assert!(matches!(t![box], Token::Keyword(Keyword::Box)));
    assert!(matches!(t![break], Token::Keyword(Keyword::Break)));
    assert!(matches!(t![const], Token::Keyword(Keyword::Const)));
    assert!(matches!(t![continue], Token::Keyword(Keyword::Continue)));
    assert!(matches!(t![crate], Token::Keyword(Keyword::Crate)));
    assert!(matches!(t![default], Token::Keyword(Keyword::Default)));
    assert!(matches!(t![do], Token::Keyword(Keyword::Do)));
    assert!(matches!(t![dyn], Token::Keyword(Keyword::Dyn)));
    assert!(matches!(t![else], Token::Keyword(Keyword::Else)));
    assert!(matches!(t![enum], Token::Keyword(Keyword::Enum)));
    assert!(matches!(t![extern], Token::Keyword(Keyword::Extern)));
    assert!(matches!(t![final], Token::Keyword(Keyword::Final)));
    assert!(matches!(t![fn], Token::Keyword(Keyword::Fn)));
    assert!(matches!(t![for], Token::Keyword(Keyword::For)));
    assert!(matches!(t![if], Token::Keyword(Keyword::If)));
    assert!(matches!(t![impl], Token::Keyword(Keyword::Impl)));
    assert!(matches!(t![in], Token::Keyword(Keyword::In)));
    assert!(matches!(t![let], Token::Keyword(Keyword::Let)));
    assert!(matches!(t![loop], Token::Keyword(Keyword::Loop)));
    assert!(matches!(t![macro], Token::Keyword(Keyword::Macro)));
    assert!(matches!(t![match], Token::Keyword(Keyword::Match)));
    assert!(matches!(t![mod], Token::Keyword(Keyword::Mod)));
    assert!(matches!(t![move], Token::Keyword(Keyword::Move)));
    assert!(matches!(t![mut], Token::Keyword(Keyword::Mut)));
    assert!(matches!(t![override], Token::Keyword(Keyword::Override)));
    assert!(matches!(t![priv], Token::Keyword(Keyword::Priv)));
    assert!(matches!(t![pub], Token::Keyword(Keyword::Pub)));
    assert!(matches!(t![ref], Token::Keyword(Keyword::Ref)));
    assert!(matches!(t![return], Token::Keyword(Keyword::Return)));
    assert!(matches!(t![Self], Token::Keyword(Keyword::SelfType)));
    assert!(matches!(t![self], Token::Keyword(Keyword::SelfValue)));
    assert!(matches!(t![static], Token::Keyword(Keyword::Static)));
    assert!(matches!(t![struct], Token::Keyword(Keyword::Struct)));
    assert!(matches!(t![super], Token::Keyword(Keyword::Super)));
    assert!(matches!(t![trait], Token::Keyword(Keyword::Trait)));
    assert!(matches!(t![try], Token::Keyword(Keyword::Try)));
    assert!(matches!(t![type], Token::Keyword(Keyword::Type)));
    assert!(matches!(t![typeof], Token::Keyword(Keyword::Typeof)));
    assert!(matches!(t![union], Token::Keyword(Keyword::Union)));
    assert!(matches!(t![unsafe], Token::Keyword(Keyword::Unsafe)));
    assert!(matches!(t![unsized], Token::Keyword(Keyword::Unsized)));
    assert!(matches!(t![use], Token::Keyword(Keyword::Use)));
    assert!(matches!(t![virtual], Token::Keyword(Keyword::Virtual)));
    assert!(matches!(t![where], Token::Keyword(Keyword::Where)));
    assert!(matches!(t![while], Token::Keyword(Keyword::While)));
    assert!(matches!(t![yield], Token::Keyword(Keyword::Yield)));
}

#[test]
fn test_tt_ident() {
    assert_matches_sym!(t![#ident], Token::Ident("ident"));
    assert_matches_sym!(t![#some_thing], Token::Ident("some_thing"));
    assert_matches_sym!(t![#SomeThing], Token::Ident("SomeThing"));
    assert!(!matches!(t![struct], Token::Ident(_)));
    assert!(!matches!(t![something], Token::Ident(_)));
}

#[test]
fn test_tt_literals() {
    assert_eq!(
        t!["string literal"],
        Token::Literal(Literal::parse("\"string literal\"").unwrap())
    );
}
