use sin::*;

#[test]
fn test_enclose() {
    assert_eq!(GroupPunct::Brace.enclose("test"), "{ test }");
    assert_eq!(GroupPunct::Bracket.enclose("test"), "[ test ]");
    assert_eq!(GroupPunct::Paren.enclose("test"), "( test )");
}
