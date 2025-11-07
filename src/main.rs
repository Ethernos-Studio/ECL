mod token;
mod ast;
mod lexer;
mod parser;
mod interpreter;
mod error;
mod repl;

use std::env;
use std::fs;
use interpreter::Interpreter;
use repl::Repl;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        // 启动REPL模式
        let mut repl = Repl::new();
        repl.run();
    } else if args[1] == "--debug-lexer" && args.len() >= 3 {
        // 调试lexer模式
        let input = if args[2].ends_with(".ecl") {
            // 如果是.ecl文件，读取文件内容
            match fs::read_to_string(&args[2]) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file {}: {}", args[2], e);
                    return;
                }
            }
        } else {
            // 否则直接使用参数
            args[2].clone()
        };
        
        let mut lexer = crate::lexer::Lexer::new(&input);
        
        println!("Debugging lexer for input: '{}'", input);
        loop {
            let (line, column) = lexer.get_position();
            let token = lexer.next_token();
            println!("Position: ({}, {}), Token: {:?}", line, column, token);
            
            if matches!(token, crate::token::Token::Eof) {
                break;
            }
        }
    } else {
        // 文件模式
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(contents) => {
                let source_lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
                let mut interpreter = Interpreter::new()
                    .with_source(filename.to_string(), source_lines);
                interpreter.run(&contents);
            }
            Err(e) => {
                eprintln!("Error reading file {}: {}", filename, e);
            }
        }
    }
}