use core::panic;

use super::token::{create_token, Token, TokenType};

pub struct Lexer {
    pub input: String,
    pub current_position: usize,
    pub read_position: usize,
    pub current_char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: String::from(input),
            current_position: 0,
            read_position: 0,
            current_char: '\0',
        };
        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.current_char {
            '=' => create_token(TokenType::ASSIGN, self.current_char),
            '+' => create_token(TokenType::PLUS, self.current_char),
            '(' => create_token(TokenType::LPAREN, self.current_char),
            ')' => create_token(TokenType::RPAREN, self.current_char),
            '{' => create_token(TokenType::LBRACE, self.current_char),
            '}' => create_token(TokenType::RBRACE, self.current_char),
            _ => create_token(TokenType::ILLEGAL, self.current_char),
        };

        self.read_char();

        token
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = '\0';
        } else {
            let char_at_read_position = self.input.chars().nth(self.read_position);
            if let Some(char_at) = char_at_read_position {
                self.current_char = char_at;
            } else {
                panic!("Lexing error, should not happen");
            }
        }
        self.current_position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::lang::token::create_token_str;

    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){}";

        let mut lexer = Lexer::new(input);

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

    #[test]
    fn test_next_token_with_literal_and_function() {
        let input = r#"let five = 5
            let ten = 10

            fun add(x, y) {
                return x + y
            }

            let result = add(five, ten)"#;

        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            // Line 5
            create_token_str(TokenType::LET, "let"),
            create_token_str(TokenType::IDENTIFIER, "five"),
            create_token_str(TokenType::ASSIGN, "="),
            create_token_str(TokenType::INT, "5"),
            create_token_str(TokenType::EOL, ""),
            // Line 10
            create_token_str(TokenType::LET, "let"),
            create_token_str(TokenType::IDENTIFIER, "ten"),
            create_token_str(TokenType::ASSIGN, "="),
            create_token_str(TokenType::INT, "10"),
            create_token_str(TokenType::EOL, ""),
            create_token_str(TokenType::EOL, ""),
            // Line add
            create_token_str(TokenType::FUNCTION, "fun"),
            create_token_str(TokenType::IDENTIFIER, "add"),
            create_token_str(TokenType::LPAREN, "("),
            create_token_str(TokenType::IDENTIFIER, "x"),
            create_token_str(TokenType::COMMA, ","),
            create_token_str(TokenType::IDENTIFIER, "y"),
            create_token_str(TokenType::RPAREN, ")"),
            create_token_str(TokenType::LBRACE, "{"),
            create_token_str(TokenType::EOL, ""),
            // Line return
            create_token_str(TokenType::RETURN, "return"),
            create_token_str(TokenType::IDENTIFIER, "x"),
            create_token_str(TokenType::PLUS, "+"),
            create_token_str(TokenType::IDENTIFIER, "y"),
            create_token_str(TokenType::EOL, ""),
            // Line result
            create_token_str(TokenType::LET, "let"),
            create_token_str(TokenType::IDENTIFIER, "result"),
            create_token_str(TokenType::ASSIGN, "="),
            create_token_str(TokenType::IDENTIFIER, "add"),
            create_token_str(TokenType::LPAREN, "("),
            create_token_str(TokenType::IDENTIFIER, "five"),
            create_token_str(TokenType::COMMA, ","),
            create_token_str(TokenType::IDENTIFIER, "ten"),
            create_token_str(TokenType::RPAREN, ")"),
        ];
    }
}
