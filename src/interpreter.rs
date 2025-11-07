use std::collections::HashMap;
use crate::ast::{ASTNode, Type};
use crate::error::format_undefined_identifier_error;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    Float(f32),
    Double(f64),
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Str(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Double(d) => d.to_string(),
        }
    }
    
    fn to_f64(&self) -> f64 {
        match self {
            Value::Int(n) => *n as f64,
            Value::Str(s) => s.parse().unwrap_or(0.0),
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
            Value::Float(f) => *f as f64,
            Value::Double(d) => *d,
        }
    }
    
    fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Str(_) => Type::Str,
            Value::Bool(_) => Type::Bool,
            Value::Float(_) => Type::Float,
            Value::Double(_) => Type::Double,
        }
    }
    
    fn convert_to(&self, target_type: &Type) -> Result<Value, String> {
        match (self, target_type) {
            // Same type - no conversion needed
            (Value::Int(_), Type::Int) => Ok(self.clone()),
            (Value::Str(_), Type::Str) => Ok(self.clone()),
            (Value::Bool(_), Type::Bool) => Ok(self.clone()),
            (Value::Float(_), Type::Float) => Ok(self.clone()),
            (Value::Double(_), Type::Double) => Ok(self.clone()),
            
            // Int to other types
            (Value::Int(n), Type::Float) => Ok(Value::Float(*n as f32)),
            (Value::Int(n), Type::Double) => Ok(Value::Double(*n as f64)),
            (Value::Int(n), Type::Str) => Ok(Value::Str(n.to_string())),
            (Value::Int(n), Type::Bool) => Ok(Value::Bool(*n != 0)),
            
            // Float to other types
            (Value::Float(f), Type::Int) => Ok(Value::Int(*f as i64)),
            (Value::Float(f), Type::Double) => Ok(Value::Double(*f as f64)),
            (Value::Float(f), Type::Str) => Ok(Value::Str(f.to_string())),
            (Value::Float(f), Type::Bool) => Ok(Value::Bool(*f != 0.0)),
            
            // Double to other types
            (Value::Double(d), Type::Int) => Ok(Value::Int(*d as i64)),
            (Value::Double(d), Type::Float) => Ok(Value::Float(*d as f32)),
            (Value::Double(d), Type::Str) => Ok(Value::Str(d.to_string())),
            (Value::Double(d), Type::Bool) => Ok(Value::Bool(*d != 0.0)),
            
            // String to other types
            (Value::Str(s), Type::Int) => {
                s.parse::<i64>()
                    .map(Value::Int)
                    .map_err(|_| format!("Cannot convert string '{}' to int", s))
            }
            (Value::Str(s), Type::Float) => {
                s.parse::<f32>()
                    .map(Value::Float)
                    .map_err(|_| format!("Cannot convert string '{}' to float", s))
            }
            (Value::Str(s), Type::Double) => {
                s.parse::<f64>()
                    .map(Value::Double)
                    .map_err(|_| format!("Cannot convert string '{}' to double", s))
            }
            (Value::Str(s), Type::Bool) => {
                match s.to_lowercase().as_str() {
                    "true" | "1" => Ok(Value::Bool(true)),
                    "false" | "0" => Ok(Value::Bool(false)),
                    _ => Err(format!("Cannot convert string '{}' to bool", s)),
                }
            }
            
            // Bool to other types
            (Value::Bool(b), Type::Int) => Ok(Value::Int(if *b { 1 } else { 0 })),
            (Value::Bool(b), Type::Float) => Ok(Value::Float(if *b { 1.0 } else { 0.0 })),
            (Value::Bool(b), Type::Double) => Ok(Value::Double(if *b { 1.0 } else { 0.0 })),
            (Value::Bool(b), Type::Str) => Ok(Value::Str(b.to_string())),
            
            // Int to Int (already handled above, but keeping for completeness)
            _ => Err(format!("Cannot convert {:?} to {:?}", self.get_type(), target_type)),
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<ASTNode>,
    pub is_expr: bool,
    pub param_types: Vec<String>,
}

pub struct Interpreter {
    variables: HashMap<String, (Value, Option<Type>)>,
    functions: HashMap<String, Function>,
    output_buffer: String,
    file_path: String,
    source_lines: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
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
                let value = self.evaluate_value(expr);
                self.variables.insert(name.clone(), (value, None));
            }
            ASTNode::TypedVar(name, var_type, expr) => {
                let value = self.evaluate_value(expr);
                // Type check and convert if needed
                match value.convert_to(var_type) {
                    Ok(converted_value) => {
                        self.variables.insert(name.clone(), (converted_value, Some(var_type.clone())));
                    }
                    Err(error_msg) => {
                        eprintln!("Type error: {}", error_msg);
                        std::process::exit(1);
                    }
                }
            }
            ASTNode::Assign(name, expr) => {
                let new_value = self.evaluate_value(expr);
                
                // Check if variable exists and has a type
                if let Some((_, var_type)) = self.variables.get(name) {
                    if let Some(expected_type) = var_type {
                        // Type check and convert if needed
                        match new_value.convert_to(expected_type) {
                            Ok(converted_value) => {
                                self.variables.insert(name.clone(), (converted_value, Some(expected_type.clone())));
                            }
                            Err(error_msg) => {
                                eprintln!("Type error in assignment: {}", error_msg);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        // Untyped variable, just assign
                        self.variables.insert(name.clone(), (new_value, None));
                    }
                } else {
                    // Variable doesn't exist, create it without type
                    self.variables.insert(name.clone(), (new_value, None));
                }
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
                            self.variables.insert(var.clone(), (Value::Int(i as i64), None));
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
                                self.variables.insert(var.clone(), (Value::Int(i as i64), None));
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
                            self.variables.insert(var.clone(), (Value::Int(i as i64), None));
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
                        self.variables.insert(var.clone(), (Value::Int(i as i64), None));
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
            ASTNode::Function(name, params, body) => {
                let function = Function {
                    params: params.clone(),
                    body: body.to_vec(),
                    is_expr: false,
                    param_types: Vec::new(),
                };
                self.functions.insert(name.clone(), function);
            }
            ASTNode::Expr(name, typed_params, body) => {
                let param_names: Vec<String> = typed_params.iter().map(|(_, name)| name.clone()).collect();
                let param_types: Vec<String> = typed_params.iter().map(|(ptype, _)| ptype.clone()).collect();
                let function = Function {
                    params: param_names,
                    body: body.to_vec(),
                    is_expr: true,
                    param_types,
                };
                self.functions.insert(name.clone(), function);
            }
            ASTNode::FunctionCall(name, args) => {
                self.execute_function_call(name, args);
            }
            ASTNode::Return(expr) => {
                let return_value = self.evaluate_value(expr);
                // For now, we'll store the return value in a special variable
                self.variables.insert("__return_value__".to_string(), (return_value, None));
            }
            _ => {}
        }
    }
    
    fn evaluate_value(&mut self, node: &ASTNode) -> Value {
        match node {
            ASTNode::Number(n) => Value::Double(*n),
            ASTNode::String(s) => Value::Str(s.clone()),
            ASTNode::Bool(b) => Value::Bool(*b),
            ASTNode::Identifier(name) => {
                self.variables.get(name).map(|(v, _)| v.clone()).unwrap_or_else(|| {
                    // Check if it's a return value from a function
                    if name == "__return_value__" {
                        return Value::Double(0.0);
                    }
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
            ASTNode::FunctionCall(name, args) => {
                self.execute_function_call(name, args);
                // Return the function's return value, or 0.0 if no return
                self.variables.get("__return_value__").map(|(v, _)| v.clone()).unwrap_or(Value::Double(0.0))
            }
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.evaluate_value(left);
                let right_val = self.evaluate_value(right);
                
                // Handle string concatenation
                if op == "+" {
                    match (&left_val, &right_val) {
                        (Value::Str(s1), Value::Str(s2)) => {
                            return Value::Str(format!("{}{}", s1, s2));
                        }
                        (Value::Str(s), other) => {
                            return Value::Str(format!("{}{}", s, other.to_string()));
                        }
                        (other, Value::Str(s)) => {
                            return Value::Str(format!("{}{}", other.to_string(), s));
                        }
                        _ => {}
                    }
                }
                
                // For other operations, convert to f64
                let left_f64 = left_val.to_f64();
                let right_f64 = right_val.to_f64();
                
                match op.as_str() {
                    "+" => Value::Double(left_f64 + right_f64),
                    "-" => Value::Double(left_f64 - right_f64),
                    "*" => Value::Double(left_f64 * right_f64),
                    "/" => {
                        if right_f64 != 0.0 {
                            Value::Double(left_f64 / right_f64)
                        } else {
                            Value::Double(0.0)
                        }
                    }
                    "<" => Value::Bool(left_f64 < right_f64),
                    ">" => Value::Bool(left_f64 > right_f64),
                    _ => Value::Double(0.0),
                }
            }
            _ => Value::Double(0.0),
        }
    }
    
    fn evaluate_print_expression(&mut self, node: &ASTNode) -> String {
        match node {
            ASTNode::String(s) => s.clone(),
            ASTNode::Number(n) => n.to_string(),
            ASTNode::Bool(b) => b.to_string(),
            ASTNode::Identifier(name) => {
                self.variables.get(name).map(|(v, _)| v.to_string()).unwrap_or_else(|| {
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
                let left_val = self.evaluate_value(left);
                let right_val = self.evaluate_value(right);
                
                // Handle string concatenation
                if op == "+" {
                    match (&left_val, &right_val) {
                        (Value::Str(s1), Value::Str(s2)) => {
                            return format!("{}{}", s1, s2);
                        }
                        (Value::Str(s), other) => {
                            return format!("{}{}", s, other.to_string());
                        }
                        (other, Value::Str(s)) => {
                            return format!("{}{}", other.to_string(), s);
                        }
                        _ => {}
                    }
                }
                
                // For other operations, convert to f64
                let left_f64 = left_val.to_f64();
                let right_f64 = right_val.to_f64();
                
                match op.as_str() {
                    "+" => (left_f64 + right_f64).to_string(),
                    "-" => (left_f64 - right_f64).to_string(),
                    "*" => (left_f64 * right_f64).to_string(),
                    "/" => {
                        if right_f64 != 0.0 {
                            (left_f64 / right_f64).to_string()
                        } else {
                            "0".to_string()
                        }
                    }
                    "<" => (left_f64 < right_f64).to_string(),
                    ">" => (left_f64 > right_f64).to_string(),
                    _ => "0".to_string(),
                }
            }
            ASTNode::FunctionCall(name, args) => {
                self.execute_function_call(name, args);
                // Return the function's return value, or "0" if no return
                self.variables.get("__return_value__").map(|(v, _)| v.to_string()).unwrap_or_else(|| "0".to_string())
            }
            _ => "0".to_string(),
        }
    }
    
    fn evaluate_expression(&mut self, node: &ASTNode) -> f64 {
        self.evaluate_value(node).to_f64()
    }
    
    fn execute_function_call(&mut self, name: &str, args: &[ASTNode]) {
        if let Some(function) = self.functions.get(name).cloned() {
            if args.len() != function.params.len() {
                eprintln!("Error: Function {} expects {} arguments, got {}", name, function.params.len(), args.len());
                return;
            }
            
            // Save current variable scope
            let saved_variables = self.variables.clone();
            
            // Set function parameters
            for (param, arg) in function.params.iter().zip(args.iter()) {
                let value = self.evaluate_value(arg);
                self.variables.insert(param.clone(), (value, None));
            }
            
            // Execute function body
            for stmt in function.body.iter() {
                self.evaluate(stmt);
                
                // Check if we have a return value
                if self.variables.contains_key("__return_value__") {
                    break;
                }
            }
            
            // Restore variable scope (but keep return value if it exists)
            let return_value = self.variables.get("__return_value__").map(|(v, _)| v.clone());
            self.variables = saved_variables;
            if let Some(value) = return_value {
                self.variables.insert("__return_value__".to_string(), (value, None));
            }
        } else {
            eprintln!("Error: Function '{}' is not defined", name);
        }
    }
}