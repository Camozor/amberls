use std::io;

use crate::lang::token::TokenType;

use super::lexer::Lexer;

pub fn start() {
    let stdin = io::stdin();

    loop {
        println!(">>>>> ");
        let mut buffer = String::new();

        let read = stdin.read_line(&mut buffer);
        if let Ok(_) = read {
            let mut lexer = Lexer::new(buffer.as_str());
            let mut token = lexer.next_token();
            while token.token_type != TokenType::EOF {
                println!("{:?}: {}", token.token_type, token.literal);
                token = lexer.next_token();
            }
        }
    }
}
