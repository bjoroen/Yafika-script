use crate::Yafika;

#[test]
fn test_lexer() {
    let lexer = Lexer::new("let hello = \"hello\"");
    lexer.lex();

    assert!(lexer.tokens.contains(Token::Let), true);
}
