use core::panic;

use super::token::{
    create_token, create_token_char, create_token_string, lookup_identifier, Token, TokenType,
};

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
        self.skip_whitespace();

        let token = match self.current_char {
            '=' => create_token_char(TokenType::ASSIGN, self.current_char),
            '+' => create_token_char(TokenType::PLUS, self.current_char),
            '(' => create_token_char(TokenType::LPAREN, self.current_char),
            ')' => create_token_char(TokenType::RPAREN, self.current_char),
            '{' => create_token_char(TokenType::LBRACE, self.current_char),
            '}' => create_token_char(TokenType::RBRACE, self.current_char),
            ',' => create_token_char(TokenType::COMMA, self.current_char),
            '\r' => create_token(TokenType::EOL, "EOL"),
            '\n' => create_token(TokenType::EOL, "EOL"),
            _ => {
                if is_letter(self.current_char) {
                    let identifier = self.read_identifier();
                    return lookup_identifier(identifier);
                } else if is_digit(self.current_char) {
                    let integer = self.read_integer();
                    return create_token_string(TokenType::INT, integer);
                } else {
                    create_token(TokenType::ILLEGAL, "")
                }
            }
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

    pub fn read_identifier(&mut self) -> String {
        let start_position = self.current_position;

        while is_letter(self.current_char) {
            self.read_char();
        }

        self.input
            .chars()
            .skip(start_position)
            .take(self.current_position - start_position)
            .collect()
    }

    pub fn read_integer(&mut self) -> String {
        let start_position = self.current_position;

        while is_digit(self.current_char) {
            self.read_char();
        }
        self.input
            .chars()
            .skip(start_position)
            .take(self.current_position - start_position)
            .collect()
    }

    pub fn skip_whitespace(&mut self) {
        while self.current_char == ' ' || self.current_char == '\t' {
            self.read_char();
        }
    }
}

fn is_letter(char: char) -> bool {
    char >= 'a' && char <= 'z' || char >= 'A' && char <= 'Z' || char == '_'
}

fn is_digit(char: char) -> bool {
    char >= '0' && char <= '9'
}

#[cfg(test)]
mod tests {
    use crate::lang::token::create_token;

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
        let input = r#"let  five = 5
            let ten = 10

            fun add(x, y) {
                return x + y
            }

            let result = add(five, ten)"#;

        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            // Line 5
            create_token(TokenType::LET, "let"),
            create_token(TokenType::IDENTIFIER, "five"),
            create_token(TokenType::ASSIGN, "="),
            create_token(TokenType::INT, "5"),
            create_token(TokenType::EOL, "EOL"),
            // Line 10
            create_token(TokenType::LET, "let"),
            create_token(TokenType::IDENTIFIER, "ten"),
            create_token(TokenType::ASSIGN, "="),
            create_token(TokenType::INT, "10"),
            create_token(TokenType::EOL, "EOL"),
            create_token(TokenType::EOL, "EOL"),
            // Line add
            create_token(TokenType::FUNCTION, "fun"),
            create_token(TokenType::IDENTIFIER, "add"),
            create_token(TokenType::LPAREN, "("),
            create_token(TokenType::IDENTIFIER, "x"),
            create_token(TokenType::COMMA, ","),
            create_token(TokenType::IDENTIFIER, "y"),
            create_token(TokenType::RPAREN, ")"),
            create_token(TokenType::LBRACE, "{"),
            create_token(TokenType::EOL, "EOL"),
            // Line return
            create_token(TokenType::RETURN, "return"),
            create_token(TokenType::IDENTIFIER, "x"),
            create_token(TokenType::PLUS, "+"),
            create_token(TokenType::IDENTIFIER, "y"),
            create_token(TokenType::EOL, "EOL"),
            create_token(TokenType::RBRACE, "}"),
            create_token(TokenType::EOL, "EOL"),
            create_token(TokenType::EOL, "EOL"),
            // Line result
            create_token(TokenType::LET, "let"),
            create_token(TokenType::IDENTIFIER, "result"),
            create_token(TokenType::ASSIGN, "="),
            create_token(TokenType::IDENTIFIER, "add"),
            create_token(TokenType::LPAREN, "("),
            create_token(TokenType::IDENTIFIER, "five"),
            create_token(TokenType::COMMA, ","),
            create_token(TokenType::IDENTIFIER, "ten"),
            create_token(TokenType::RPAREN, ")"),
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
            println!("token literal={}.", token.literal);
        }
    }
}
