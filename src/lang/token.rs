pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,

    ASSIGN,
    PLUS,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    LET,
}
