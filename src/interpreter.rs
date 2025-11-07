use std::collections::HashMap;
use crate::ast::ASTNode;
use crate::error::format_undefined_identifier_error;

pub struct Interpreter {
    variables: HashMap<String, f64>,
    output_buffer: String,
    file_path: String,
    source_lines: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            output_buffer: String::new(),
            file_path: String::from("<unknown>"),
            source_lines: Vec::new(),
        }
    }
    
    pub fn with_source(mut self, file_path: String, source_lines: Vec<String>) -> Self {
        self.file_path = file_path;
        self.source_lines = source_lines;
        self
    }
    
    pub fn run(&mut self, input: &str) {
        use crate::lexer::Lexer;
        use crate::parser::Parser;
        
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        
        // 如果没有设置源文件信息，从输入中提取
        if self.source_lines.is_empty() {
            self.source_lines = input.lines().map(|s| s.to_string()).collect();
        }
        
        match parser.parse(&self.file_path, &self.source_lines) {
            Ok(ast) => {
                for node in ast {
                    self.evaluate(&node);
                }
                
                if !self.output_buffer.is_empty() {
                    println!("{}", self.output_buffer);
                    self.output_buffer.clear();
                }
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
    
    fn evaluate(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Print(expr) => {
                let value = self.evaluate_print_expression(expr);
                self.output_buffer.push_str(&value);
            }
            ASTNode::Println(expr) => {
                let value = self.evaluate_print_expression(expr);
                if !self.output_buffer.is_empty() {
                    self.output_buffer.push_str(&value);
                    println!("{}", self.output_buffer);
                    self.output_buffer.clear();
                } else {
                    println!("{}", value);
                }
            }
            ASTNode::Var(name, expr) => {
                let value = self.evaluate_expression(expr);
                self.variables.insert(name.clone(), value);
            }
            ASTNode::Assign(name, expr) => {
                let value = self.evaluate_expression(expr);
                self.variables.insert(name.clone(), value);
            }
            ASTNode::For(var, range_expr, body) => {
                // 解析range表达式，支持 range(start, end) 或 start..end 格式
                if let ASTNode::BinaryOp(left, op, right) = range_expr.as_ref() {
                    if op == ".." {
                        // 处理 start..end 格式
                        let start_val = self.evaluate_expression(left);
                        let end_val = self.evaluate_expression(right);
                        
                        let mut i = start_val as i32;
                        while i < end_val as i32 {
                            self.variables.insert(var.clone(), i as f64);
                            for stmt in body.iter() {
                                self.evaluate(stmt);
                            }
                            i += 1;
                        }
                    } else if let ASTNode::Identifier(func_name) = range_expr.as_ref() {
                        if func_name == "range" {
                            // 这里需要更复杂的逻辑来处理 range(start, end)
                            // 暂时使用简单的实现
                            let start_val = 0.0; // 默认值
                            let end_val = 10.0;  // 默认值
                            
                            let mut i = start_val as i32;
                            while i < end_val as i32 {
                                self.variables.insert(var.clone(), i as f64);
                                for stmt in body.iter() {
                                    self.evaluate(stmt);
                                }
                                i += 1;
                            }
                        }
                    } else {
                        // 默认处理，假设是一个范围表达式
                        let range_val = self.evaluate_expression(range_expr);
                        let start_val = 0.0;
                        let end_val = range_val;
                        
                        let mut i = start_val as i32;
                        while i < end_val as i32 {
                            self.variables.insert(var.clone(), i as f64);
                            for stmt in body.iter() {
                                self.evaluate(stmt);
                            }
                            i += 1;
                        }
                    }
                } else {
                    // 默认处理
                    let range_val = self.evaluate_expression(range_expr);
                    let start_val = 0.0;
                    let end_val = range_val;
                    
                    let mut i = start_val as i32;
                    while i < end_val as i32 {
                        self.variables.insert(var.clone(), i as f64);
                        for stmt in body.iter() {
                            self.evaluate(stmt);
                        }
                        i += 1;
                    }
                }
            }
            ASTNode::If(condition, then_branch, else_branch) => {
                let cond_val = self.evaluate_expression(condition);
                if cond_val != 0.0 {
                    for stmt in then_branch.iter() {
                        self.evaluate(stmt);
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts.iter() {
                        self.evaluate(stmt);
                    }
                }
            }
            _ => {}
        }
    }
    
    fn evaluate_print_expression(&self, node: &ASTNode) -> String {
        match node {
            ASTNode::String(s) => s.clone(),
            ASTNode::Number(n) => n.to_string(),
            ASTNode::Identifier(name) => {
                self.variables.get(name).map(|v| v.to_string()).unwrap_or_else(|| {
                    // 获取当前位置信息（这里使用1,1作为默认值，实际需要更精确的位置）
                    let error = format_undefined_identifier_error(
                        name,
                        1, // 需要获取实际行号
                        1, // 需要获取实际列号
                        &self.file_path,
                        self.source_lines.get(0).unwrap_or(&String::new())
                    );
                    eprintln!("{}", error);
                    std::process::exit(1);
                })
            }
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left);
                let right_val = self.evaluate_expression(right);
                
                match op.as_str() {
                    "+" => (left_val + right_val).to_string(),
                    "-" => (left_val - right_val).to_string(),
                    "*" => (left_val * right_val).to_string(),
                    "/" => {
                        if right_val != 0.0 {
                            (left_val / right_val).to_string()
                        } else {
                            "0".to_string()
                        }
                    }
                    "<" => (left_val < right_val).to_string(),
                    ">" => (left_val > right_val).to_string(),
                    _ => "0".to_string(),
                }
            }
            _ => "0".to_string(),
        }
    }
    
    fn evaluate_expression(&self, node: &ASTNode) -> f64 {
        match node {
            ASTNode::Number(n) => *n,
            ASTNode::Identifier(name) => self.variables.get(name).copied().unwrap_or_else(|| {
                // 获取当前位置信息（这里使用1,1作为默认值，实际需要更精确的位置）
                let error = format_undefined_identifier_error(
                    name,
                    1, // 需要获取实际行号
                    1, // 需要获取实际列号
                    &self.file_path,
                    self.source_lines.get(0).unwrap_or(&String::new())
                );
                eprintln!("{}", error);
                std::process::exit(1);
            }),
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_expression(left);
                let right_val = self.evaluate_expression(right);
                
                match op.as_str() {
                    "+" => left_val + right_val,
                    "-" => left_val - right_val,
                    "*" => left_val * right_val,
                    "/" => {
                        if right_val != 0.0 {
                            left_val / right_val
                        } else {
                            0.0
                        }
                    }
                    "<" => (left_val < right_val) as i32 as f64,
                    ">" => (left_val > right_val) as i32 as f64,
                    _ => 0.0,
                }
            }
            ASTNode::String(_) => 0.0,
            _ => 0.0,
        }
    }
}