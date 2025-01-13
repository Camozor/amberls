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

pub fn create_token(token_type: TokenType, literal: &str) -> Token {
    Token {
        token_type,
        literal: format!("{}", literal),
    }
}

pub fn create_token_string(token_type: TokenType, literal: String) -> Token {
    Token {
        token_type,
        literal: format!("{}", literal),
    }
}

pub fn create_token_char(token_type: TokenType, char: char) -> Token {
    Token {
        token_type,
        literal: char.to_string(),
    }
}

pub fn lookup_identifier(identifier: String) -> Token {
    if identifier == "fun" {
        return Token {
            token_type: TokenType::FUNCTION,
            literal: format!("fun"),
        };
    } else if identifier == "let" {
        return Token {
            token_type: TokenType::LET,
            literal: format!("let"),
        };
    } else if identifier == "return" {
        return Token {
            token_type: TokenType::RETURN,
            literal: format!("return"),
        };
    }

    create_token_string(TokenType::IDENTIFIER, identifier)
}
