use sin::*;

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
    // assert!(matches!(tt![>=], Token::Punct(Punct::Ge))); // bad
}

#[test]
fn test_tt_ident() {
    assert!(matches!(tt![something], Token::Ident(_)));
    assert!(matches!(tt![struct], Token::Ident(_))); // bad
}
