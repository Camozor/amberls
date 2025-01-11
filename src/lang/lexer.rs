use super::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,
    pub position: i32,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: String::from(input),
            position: 0,
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

        let expected_tokens = vec![
            Token {
                token_type: TokenType::ASSIGN,
                literal: String::from("="),
            },
            Token {
                token_type: TokenType::PLUS,
                literal: String::from("+"),
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: String::from("("),
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: String::from(")"),
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: String::from("{"),
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: String::from("}"),
            },
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
        }
    }
}
