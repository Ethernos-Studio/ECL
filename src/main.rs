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