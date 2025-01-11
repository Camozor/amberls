pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    EOL,

    IDENTIFIER,
    INT,

    ASSIGN,
    PLUS,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,

    LET,
    FUNCTION,
    RETURN,
}

pub fn create_token_str(token_type: TokenType, literal: &str) -> Token {
    Token {
        token_type,
        literal: format!("{}", literal),
    }
}

pub fn create_token(token_type: TokenType, ch: char) -> Token {
    Token {
        token_type,
        literal: ch.to_string(),
    }
}
