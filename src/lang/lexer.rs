use super::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: String::from(input),
        }
    }

    pub fn next_token(&self) -> Token {
        Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){}";

        let lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_eq!(token.token_type, TokenType::ASSIGN)
    }
}
