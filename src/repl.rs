use std::io::{self, Write};
use crate::interpreter::Interpreter;

pub struct Repl {
    interpreter: Interpreter,
    history: Vec<String>,
    prompt: String,
    multiline_buffer: String,
    is_multiline: bool,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            history: Vec::new(),
            prompt: String::from("ecl> "),
            multiline_buffer: String::new(),
            is_multiline: false,
        }
    }
    
    pub fn run(&mut self) {
        println!("╔═══════════════════════════════════════╗");
        println!("║     ECL (ECL Command Language)       ║");
        println!("║              REPL                    ║");
        println!("╚═══════════════════════════════════════╝");
        println!();
        println!("💡 Type 'help' for help, 'exit' to quit");
        println!("📝 Use '{{' to start multiline input, '}}' to end");
        println!("🔍 Auto-completion: parentheses and braces are balanced");
        println!();
        
        loop {
            let prompt = if self.is_multiline {
                "...> "
            } else {
                &self.prompt
            };
            
            print!("{}", prompt);
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();
                    if input.is_empty() && !self.is_multiline {
                        continue;
                    }
                    
                    if self.handle_multiline_input(input) {
                        break;
                    }
                }
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
                }
            }
        }
    }
    
    fn handle_multiline_input(&mut self, input: &str) -> bool {
        // 处理特殊的多行输入标记
        if input == "{{" {
            self.is_multiline = true;
            self.multiline_buffer.clear();
            println!("📝 Entering multiline mode. Type '}}' to execute.");
            return false;
        }
        
        if self.is_multiline {
            if input == "}}" {
                // 执行多行代码
                let code = self.multiline_buffer.trim().to_string();
                if !code.is_empty() {
                    self.history.push(code.clone());
                    self.execute_code(&code);
                }
                self.is_multiline = false;
                self.multiline_buffer.clear();
                return false;
            } else {
                // 添加到多行缓冲区
                self.multiline_buffer.push_str(input);
                self.multiline_buffer.push('\n');
                return false;
            }
        }
        
        // 单行模式，检查是否需要自动补全
        let balanced_input = self.auto_complete_braces(input);
        self.history.push(balanced_input.clone());
        self.handle_command(&balanced_input)
    }
    
    fn auto_complete_braces(&self, input: &str) -> String {
        let mut result = input.to_string();
        let mut paren_count = 0;
        let mut brace_count = 0;
        
        for ch in input.chars() {
            match ch {
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                _ => {}
            }
        }
        
        // 自动补全未闭合的括号
        while paren_count > 0 {
            result.push(')');
            paren_count -= 1;
        }
        
        while brace_count > 0 {
            result.push('}');
            brace_count -= 1;
        }
        
        result
    }
    
    fn handle_command(&mut self, input: &str) -> bool {
        match input {
            "exit" | "quit" => {
                println!("Goodbye!");
                true
            }
            "help" => {
                self.print_help();
                false
            }
            "history" => {
                self.print_history();
                false
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
                false
            }
            _ => {
                self.execute_code(input);
                false
            }
        }
    }
    
    fn execute_code(&mut self, code: &str) {
        // 为REPL模式设置特殊的文件路径和源代码
        let source_lines: Vec<String> = vec![code.to_string()];
        self.interpreter = Interpreter::new()
            .with_source("<repl>".to_string(), source_lines);
        
        println!("📝 Executing: {}", code);
        println!("─────────────────────────────────");
        self.interpreter.run(code);
        println!("─────────────────────────────────");
    }
    
    fn print_help(&self) {
        println!("\n📚 ECL REPL Commands:");
        println!("  help     - Show this help message");
        println!("  history  - Show command history");
        println!("  clear    - Clear the screen");
        println!("  exit     - Exit the REPL");
        println!("  {{        - Start multiline input");
        println!("  }}        - End multiline input");
        println!();
        println!("🚀 Language Features:");
        println!("  Variables: var x = 10;");
        println!("  Print: print(\"hello\"); println(42);");
        println!("  For loops: for i in 1..5 {{ print(i); }}");
        println!("  If statements: if (x > 5) {{ print(\"big\"); }}");
        println!("  Math: +, -, *, /, <, >");
        println!("  Ranges: 1..5, (i+1)..10");
        println!();
        println!("💡 Examples:");
        println!("  ecl> var x = 42;");
        println!("  ecl> print(x);");
        println!("  ecl> for i in 1..3 {{ print(i); }}");
        println!("  ecl> {{");
        println!("  ...> var sum = 0;");
        println!("  ...> for i in 1..5 {{ sum = sum + i; }}");
        println!("  ...> print(sum);");
        println!("  ...> }}");
    }
    
    fn print_history(&self) {
        if self.history.is_empty() {
            println!("📋 No history");
            return;
        }
        
        println!("\n📋 Command History:");
        for (i, command) in self.history.iter().enumerate() {
            println!("  {:3}: {}", i + 1, command);
        }
        println!();
    }
}