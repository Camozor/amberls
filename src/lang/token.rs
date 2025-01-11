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

pub fn create_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}
