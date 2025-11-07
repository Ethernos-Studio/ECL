use std::fs;

mod src {
    pub mod lexer {
        include!("src/lexer.rs");
    }
    pub mod token {
        include!("src/token.rs");
    }
}

use src::lexer::Lexer;
use src::token::Token;

fn main() {
    let input = "x2";
    let mut lexer = Lexer::new(input);
    
    loop {
        let token = lexer.next_token();
        let (line, column) = lexer.get_position();
        println!("Token: {:?}, Position: ({}, {})", token, line, column);
        
        if matches!(token, Token::Eof) {
            break;
        }
    }
}