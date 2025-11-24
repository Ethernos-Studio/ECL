use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::ast::{ASTNode, Type};
use crate::error::{error_messages, create_undefined_identifier_error, CompilerError};

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    Float(f32),
    Double(f64),
    Array(Vec<Value>),     // 固定长度的同类型数组
    List(Vec<Value>),      // 动态长度的异类型列表
}

impl Value {
    fn get_type_name(&self) -> String {
        match self {
            Value::Int(_) => "int".to_string(),
            Value::Str(_) => "str".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::Double(_) => "double".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::List(_) => "list".to_string(),
        }
    }
    
    fn to_string(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Str(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Double(d) => d.to_string(),
            Value::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            },
            Value::List(lst) => {
                let elements: Vec<String> = lst.iter().map(|v| v.to_string()).collect();
                format!("[{}]", elements.join(", "))
            },
        }
    }
    
    fn to_f64(&self) -> f64 {
        match self {
            Value::Int(n) => *n as f64,
            Value::Str(s) => s.parse().unwrap_or(0.0),
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
            Value::Float(f) => *f as f64,
            Value::Double(d) => *d,
            Value::Array(_) => 0.0,  // 数组转为数字时返回0
            Value::List(_) => 0.0,   // 列表转为数字时返回0
        }
    }
    
    fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Str(_) => Type::Str,
            Value::Bool(_) => Type::Bool,
            Value::Float(_) => Type::Float,
            Value::Double(_) => Type::Double,
            Value::Array(_) => Type::Any,  // 数组类型
            Value::List(_) => Type::Any,   // 列表类型
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
            (Value::Array(_), Type::Any) => Ok(self.clone()),  // 数组保持不变
            (Value::List(_), Type::Any) => Ok(self.clone()),   // 列表保持不变
            
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
            
            // Array/List to String
            (Value::Array(_), Type::Str) => Ok(Value::Str(self.to_string())),
            (Value::List(_), Type::Str) => Ok(Value::Str(self.to_string())),
            
            // Int to Int (already handled above, but keeping for completeness)
            _ => Err(format!("Cannot convert {:?} to {:?}", self.get_type(), target_type)),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<ASTNode>,
    pub is_expr: bool,        // 标记是否为表达式函数
    pub param_types: Vec<String>,  // 参数类型信息
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
            ASTNode::ArrayDecl(name, element_type, size, init_values) => {
                // Create an array with the specified size and initialize it
                let mut array_elements = Vec::new();
                let size = *size; // Dereference size
                
                // If we have initialization values
                if !init_values.is_empty() {
                    // Check if we have a single value to initialize all elements
                    if init_values.len() == 1 {
                        let init_value = self.evaluate_value(&init_values[0]);
                        // Convert to the specified type if needed
                        let typed_value = match init_value.convert_to(&element_type) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Type error in array initialization: {}", e);
                                std::process::exit(1);
                            }
                        };
                        
                        // Initialize all elements with this value
                        for _ in 0..size {
                            array_elements.push(typed_value.clone());
                        }
                    } else {
                        // Initialize with provided values
                        for i in 0..std::cmp::min(size, init_values.len()) {
                            let init_value = self.evaluate_value(&init_values[i]);
                            // Convert to the specified type if needed
                            let typed_value = match init_value.convert_to(&element_type) {
                                Ok(v) => v,
                                Err(e) => {
                                    eprintln!("Type error in array initialization: {}", e);
                                    std::process::exit(1);
                                }
                            };
                            array_elements.push(typed_value);
                        }
                        
                        // If we have fewer values than size, fill the rest with default values
                        if init_values.len() < size {
                            let default_value = match element_type {
                                Type::Int => Value::Int(0),
                                Type::Float => Value::Float(0.0),
                                Type::Double => Value::Double(0.0),
                                Type::Bool => Value::Bool(false),
                                Type::Str => Value::Str("".to_string()),
                                Type::Any => Value::Int(0), // Default to 0 for any type
                            };
                            
                            for _ in init_values.len()..size {
                                array_elements.push(default_value.clone());
                            }
                        }
                    }
                } else {
                    // No initialization values, initialize with default values
                    let default_value = match element_type {
                        Type::Int => Value::Int(0),
                        Type::Float => Value::Float(0.0),
                        Type::Double => Value::Double(0.0),
                        Type::Bool => Value::Bool(false),
                        Type::Str => Value::Str("".to_string()),
                        Type::Any => Value::Int(0), // Default to 0 for any type
                    };
                    
                    for _ in 0..size {
                        array_elements.push(default_value.clone());
                    }
                }
                
                // Store the array with its element type as the variable type
                self.variables.insert(name.clone(), (Value::Array(array_elements), Some(element_type.clone())));
            }
            ASTNode::ListDecl(name, init_values) => {
                // Create a list with the provided initialization values
                let mut list_elements = Vec::new();
                
                for init_value in init_values {
                    let value = self.evaluate_value(init_value);
                    list_elements.push(value);
                }
                
                self.variables.insert(name.clone(), (Value::List(list_elements), None));
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
            ASTNode::IndexAssign(array_expr, index_expr, value_expr, pos) => {
                // Handle array/list index assignment: array[index] = value
                let array_identifier = match array_expr.as_ref() {
                    ASTNode::Identifier(name, _) => name,
                    _ => return, // Not a valid array identifier
                };
                
                let index_val = self.evaluate_value(index_expr);
                let value_to_assign = self.evaluate_value(value_expr);
                let index = index_val.to_f64() as usize;
                
                // Get the current array/list from variables
                if let Some((current_value, var_type)) = self.variables.get(array_identifier).cloned() {
                    let new_value = match &current_value {
                        Value::Array(arr) => {
                            if index < arr.len() {
                                // For arrays, check type compatibility based on the variable's type
                                if let Some(expected_type) = &var_type {
                                    // For array variables, the type is the element type
                                    match value_to_assign.convert_to(expected_type) {
                                        Ok(converted_value) => {
                                            let mut new_arr = arr.clone();
                                            new_arr[index] = converted_value;
                                            Value::Array(new_arr)
                                        }
                                        Err(_) => {
                                            // Create a proper compiler error with full formatting
                                            let error_msg = error_messages::array_type_error(
                                                &expected_type.to_string(), 
                                                &value_to_assign.get_type_name()
                                            );
                                            
                                            let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                                                .unwrap_or(&String::new()).clone();
                                            
                                            let compiler_error = CompilerError::new(
                                                error_msg,
                                                pos.line,
                                                pos.column,
                                                self.file_path.clone(),
                                                source_line,
                                            )
                                            .with_help(error_messages::help_array_assignment())
                                            .with_example(error_messages::example_array_assignment());
                                            
                                            eprintln!("{}", compiler_error);
                                            std::process::exit(1);
                                        }
                                    }
                                } else {
                                    // If no type info, just assign
                                    let mut new_arr = arr.clone();
                                    new_arr[index] = value_to_assign;
                                    Value::Array(new_arr)
                                }
                            } else {
                                // Index out of bounds - ignore or handle error
                                current_value
                            }
                        }
                        Value::List(lst) => {
                            if index < lst.len() {
                                let mut new_lst = lst.clone();
                                new_lst[index] = value_to_assign;
                                Value::List(new_lst)
                            } else {
                                // Index out of bounds - ignore or handle error
                                current_value
                            }
                        }
                        _ => {
                            // Not an array or list - can't do index assignment
                            current_value
                        }
                    };
                    
                    // Update the variable with the modified array/list
                    self.variables.insert(array_identifier.clone(), (new_value, var_type));
                }
            }
            ASTNode::For(var, range_expr, body) => {
                // 解析range表达式，支持 range(start, end) 或 start..end 格式
                if let ASTNode::BinaryOp(left, op, right, _) = range_expr.as_ref() {
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
                    } else if let ASTNode::Identifier(func_name, _) = range_expr.as_ref() {
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
            ASTNode::While(condition, body) => {
                loop {
                    let cond_val = self.evaluate_expression(condition);
                    if cond_val == 0.0 {
                        break;
                    }
                    for stmt in body.iter() {
                        self.evaluate(stmt);
                    }
                }
            }
            ASTNode::Input(prompt, var_name) => {
                use std::io::{self, Write};
                
                let prompt_str = self.evaluate_print_expression(prompt);
                print!("{}", prompt_str);
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let input = input.trim();
                
                // Try to parse as number, otherwise store as string
                if let Ok(num) = input.parse::<f64>() {
                    self.variables.insert(var_name.clone(), (Value::Double(num), None));
                } else {
                    self.variables.insert(var_name.clone(), (Value::Str(input.to_string()), None));
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
            ASTNode::Import(filename, pos) => {
                // Handle import statement
                self.handle_import(filename, pos);
            }
            _ => {}
        }
    }
    
    fn evaluate_value(&mut self, node: &ASTNode) -> Value {
        match node {
            ASTNode::Number(n) => Value::Double(*n),
            ASTNode::String(s) => Value::Str(s.clone()),
            ASTNode::Bool(b) => Value::Bool(*b),
            ASTNode::Identifier(name, pos) => {
                self.variables.get(name).map(|(v, _)| v.clone()).unwrap_or_else(|| {
                    // Check if it's a return value from a function
                    if name == "__return_value__" {
                        return Value::Double(0.0);
                    }
                    // 使用标识符节点中的位置信息
                    let error = create_undefined_identifier_error(
                        name,
                        pos.line,
                        pos.column,
                        &self.file_path,
                        self.source_lines.get(pos.line.saturating_sub(1)).unwrap_or(&String::new())
                    );
                    eprintln!("{}", error);
                    std::process::exit(1);
                })
            }
            ASTNode::IndexAccess(array_expr, index_expr, pos) => {
                let array_val = self.evaluate_value(array_expr);
                let index_val = self.evaluate_value(index_expr);
                
                let index = index_val.to_f64() as usize;
                
                match array_val {
                    Value::Array(arr) => {
                        if index < arr.len() {
                            arr[index].clone()
                        } else {
                            // Index out of bounds - report error with standard compiler error format
                            let error_msg = error_messages::array_index_out_of_bounds(index, arr.len());
                            let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                                .unwrap_or(&String::new()).clone();
                            
                            let compiler_error = CompilerError::new(
                                error_msg,
                                pos.line,
                                pos.column,
                                self.file_path.clone(),
                                source_line,
                            )
                            .with_help(error_messages::help_array_bounds())
                            .with_example(error_messages::example_array_bounds());
                            
                            eprintln!("{}", compiler_error);
                            std::process::exit(1);
                        }
                    }
                    Value::List(lst) => {
                        if index < lst.len() {
                            lst[index].clone()
                        } else {
                            // Index out of bounds - report error with standard compiler error format
                            let error_msg = error_messages::list_index_out_of_bounds(index, lst.len());
                            let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                                .unwrap_or(&String::new()).clone();
                            
                            let compiler_error = CompilerError::new(
                                error_msg,
                                pos.line,
                                pos.column,
                                self.file_path.clone(),
                                source_line,
                            )
                            .with_help(error_messages::help_array_bounds())
                            .with_example(error_messages::example_array_bounds());
                            
                            eprintln!("{}", compiler_error);
                            std::process::exit(1);
                        }
                    }
                    _ => {
                        // Not an array or list - return 0
                        Value::Double(0.0)
                    }
                }
            }
            ASTNode::FunctionCall(name, args) => {
                self.execute_function_call(name, args);
                // Return the function's return value, or 0.0 if no return
                self.variables.get("__return_value__").map(|(v, _)| v.clone()).unwrap_or(Value::Double(0.0))
            }
            ASTNode::BinaryOp(left, op, right, pos) => {
                let left_val = self.evaluate_value(left);
                let right_val = self.evaluate_value(right);
                
                // Handle string concatenation - only allow string + string
                if op == "+" {
                    match (&left_val, &right_val) {
                        (Value::Str(s1), Value::Str(s2)) => {
                            return Value::Str(format!("{}{}", s1, s2));
                        }
                        // For other types, we now require explicit conversion
                        _ => {}
                    }
                }
                
                // For other operations, convert to f64
                // Check if both operands are numeric types before doing arithmetic
                match (&left_val, &right_val) {
                    (Value::Int(_) | Value::Float(_) | Value::Double(_), 
                     Value::Int(_) | Value::Float(_) | Value::Double(_)) => {
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
                            "==" => Value::Bool(left_f64 == right_f64),
                            "<" => Value::Bool(left_f64 < right_f64),
                            ">" => Value::Bool(left_f64 > right_f64),
                            _ => Value::Double(0.0),
                        }
                    }
                    // Handle string concatenation with explicit conversion requirement
                    (Value::Str(_), Value::Str(_)) if op == "+" => {
                        // This case is already handled above
                        Value::Str(format!("{}{}", left_val.to_string(), right_val.to_string()))
                    }
                    // All other combinations are type errors
                    _ => {
                        // Create a type error for incompatible operations
                        let error_msg = format!("Type error: cannot perform operation '{}' between '{}' and '{}'",
                                              op, left_val.get_type_name(), right_val.get_type_name());
                        let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                            .unwrap_or(&String::new()).clone();
                        
                        let compiler_error = CompilerError::new(
                            error_msg,
                            pos.line,
                            pos.column,
                            self.file_path.clone(),
                            source_line,
                        )
                        .with_help("help: ensure both operands are of compatible types for the operation".to_string())
                        .with_example("example: for arithmetic operations, both operands should be numeric types".to_string());
                        
                        eprintln!("{}", compiler_error);
                        std::process::exit(1);
                    }
                }
            }
            ASTNode::TypeConversion(target_type, expr, pos) => {
                let value = self.evaluate_value(expr);
                match value.convert_to(target_type) {
                    Ok(converted_value) => converted_value,
                    Err(error_msg) => {
                        // Create a proper compiler error with full formatting
                        let error_msg = format!("Type conversion failed: {}", error_msg);
                        let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                            .unwrap_or(&String::new()).clone();
                        
                        let compiler_error = CompilerError::new(
                            error_msg,
                            pos.line,
                            pos.column,
                            self.file_path.clone(),
                            source_line,
                        )
                        .with_help(error_messages::help_array_type())
                        .with_example(error_messages::example_array_type());
                        
                        eprintln!("{}", compiler_error);
                        std::process::exit(1);
                    }
                }
            }
            ASTNode::IfExpr(condition, then_expr, else_expr) => {
                let cond_val = self.evaluate_expression(condition);
                if cond_val != 0.0 {
                    self.evaluate_value(then_expr)
                } else {
                    self.evaluate_value(else_expr)
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
            ASTNode::Identifier(name, pos) => {
                self.variables.get(name).map(|(v, _)| v.to_string()).unwrap_or_else(|| {
                    // 使用标识符节点中的位置信息
                    let error = create_undefined_identifier_error(
                        name,
                        pos.line,
                        pos.column,
                        &self.file_path,
                        self.source_lines.get(pos.line.saturating_sub(1)).unwrap_or(&String::new())
                    );
                    eprintln!("{}", error);
                    std::process::exit(1);
                })
            }
            ASTNode::BinaryOp(left, op, right, pos) => {
                let left_val = self.evaluate_value(left);
                let right_val = self.evaluate_value(right);
                
                // Handle string concatenation - only allow string + string
                if op == "+" {
                    match (&left_val, &right_val) {
                        (Value::Str(s1), Value::Str(s2)) => {
                            return format!("{}{}", s1, s2);
                        }
                        // For other types, we now require explicit conversion
                        _ => {}
                    }
                }
                
                // For other operations, convert to f64
                // Check if both operands are numeric types before doing arithmetic
                match (&left_val, &right_val) {
                    (Value::Int(_) | Value::Float(_) | Value::Double(_), 
                     Value::Int(_) | Value::Float(_) | Value::Double(_)) => {
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
                            "==" => (left_f64 == right_f64).to_string(),
                            "<" => (left_f64 < right_f64).to_string(),
                            ">" => (left_f64 > right_f64).to_string(),
                            _ => "0".to_string(),
                        }
                    }
                    // Handle string concatenation with explicit conversion requirement
                    (Value::Str(_), Value::Str(_)) if op == "+" => {
                        // This case is already handled above
                        format!("{}{}", left_val.to_string(), right_val.to_string())
                    }
                    // All other combinations are type errors
                    _ => {
                        // Create a type error for incompatible operations
                        let error_msg = format!("Type error: cannot perform operation '{}' between '{}' and '{}'",
                                              op, left_val.get_type_name(), right_val.get_type_name());
                        let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                            .unwrap_or(&String::new()).clone();
                        
                        let compiler_error = CompilerError::new(
                            error_msg,
                            pos.line,
                            pos.column,
                            self.file_path.clone(),
                            source_line,
                        )
                        .with_help("help: ensure both operands are of compatible types for the operation".to_string())
                        .with_example("example: for arithmetic operations, both operands should be numeric types".to_string());
                        
                        eprintln!("{}", compiler_error);
                        std::process::exit(1);
                    }
                }
            }
            ASTNode::FunctionCall(name, args) => {
                self.execute_function_call(name, args);
                // Return the function's return value, or "0" if no return
                self.variables.get("__return_value__").map(|(v, _)| v.to_string()).unwrap_or_else(|| "0".to_string())
            }
            ASTNode::TypeConversion(target_type, expr, pos) => {
                let value = self.evaluate_value(expr);
                match value.convert_to(target_type) {
                    Ok(converted_value) => converted_value.to_string(),
                    Err(error_msg) => {
                        // Create a proper compiler error with full formatting
                        let error_msg = format!("Type conversion failed: {}", error_msg);
                        let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                            .unwrap_or(&String::new()).clone();
                        
                        let compiler_error = CompilerError::new(
                            error_msg,
                            pos.line,
                            pos.column,
                            self.file_path.clone(),
                            source_line,
                        )
                        .with_help(error_messages::help_array_type())
                        .with_example(error_messages::example_array_type());
                        
                        eprintln!("{}", compiler_error);
                        std::process::exit(1);
                    }
                }
            }
            ASTNode::IfExpr(condition, then_expr, else_expr) => {
                let cond_val = self.evaluate_expression(condition);
                if cond_val != 0.0 {
                    self.evaluate_print_expression(then_expr)
                } else {
                    self.evaluate_print_expression(else_expr)
                }
            }
            ASTNode::IndexAccess(array_expr, index_expr, pos) => {
                let array_val = self.evaluate_value(array_expr);
                let index_val = self.evaluate_value(index_expr);
                let index = index_val.to_f64() as usize;
                
                match array_val {
                    Value::Array(arr) => {
                        if index < arr.len() {
                            arr[index].to_string()
                        } else {
                            // Index out of bounds - report error
                            let error_msg = error_messages::array_index_out_of_bounds(index, arr.len());
                            let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                                .unwrap_or(&String::new()).clone();
                            
                            let compiler_error = CompilerError::new(
                                error_msg,
                                pos.line,
                                pos.column,
                                self.file_path.clone(),
                                source_line,
                            )
                            .with_help(error_messages::help_array_bounds())
                            .with_example(error_messages::example_array_bounds());
                            
                            eprintln!("{}", compiler_error);
                            std::process::exit(1);
                        }
                    }
                    Value::List(lst) => {
                        if index < lst.len() {
                            lst[index].to_string()
                        } else {
                            // Index out of bounds - report error
                            let error_msg = error_messages::list_index_out_of_bounds(index, lst.len());
                            let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                                .unwrap_or(&String::new()).clone();
                            
                            let compiler_error = CompilerError::new(
                                error_msg,
                                pos.line,
                                pos.column,
                                self.file_path.clone(),
                                source_line,
                            )
                            .with_help(error_messages::help_array_bounds())
                            .with_example(error_messages::example_array_bounds());
                            
                            eprintln!("{}", compiler_error);
                            std::process::exit(1);
                        }
                        
                    }
                    _ => {
                        // Not an array or list - return "0"
                        "0".to_string()
                    }
                }
            }
            _ => "0".to_string(),
        }
    }
    
    fn handle_import(&mut self, filename: &str, pos: &crate::ast::Position) {
        // 构建完整的文件路径
        let import_path = if filename.ends_with(".ecl") {
            filename.to_string()
        } else {
            format!("{}.ecl", filename)
        };
        
        // 尝试读取导入的文件
        match fs::read_to_string(&import_path) {
            Ok(content) => {
                // 解析并执行导入的文件内容
                use crate::lexer::Lexer;
                use crate::parser::Parser;
                
                let lexer = Lexer::new(&content);
                let mut parser = Parser::new(lexer);
                
                // 保存当前文件路径和源行
                let saved_file_path = self.file_path.clone();
                let saved_source_lines = self.source_lines.clone();
                
                // 设置新的文件路径和源行
                let import_source_lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                self.file_path = import_path.clone();
                self.source_lines = import_source_lines.clone();
                
                // 解析导入的文件
                match parser.parse(&self.file_path, &self.source_lines) {
                    Ok(ast_nodes) => {
                        // 执行导入的文件中的所有语句
                        for node in ast_nodes {
                            self.evaluate(&node);
                        }
                    }
                    Err(error) => {
                        eprintln!("{}", error);
                        std::process::exit(1);
                    }
                }
                
                // 恢复原来的文件路径和源行
                self.file_path = saved_file_path;
                self.source_lines = saved_source_lines;
            }
            Err(e) => {
                // 创建导入错误消息
                let error_msg = format!("Import error: cannot read file '{}': {}", import_path, e);
                let source_line = self.source_lines.get(pos.line.saturating_sub(1))
                    .unwrap_or(&String::new()).clone();
                
                let compiler_error = CompilerError::new(
                    error_msg,
                    pos.line,
                    pos.column,
                    self.file_path.clone(),
                    source_line,
                );
                
                eprintln!("{}", compiler_error);
                std::process::exit(1);
            }
        }
    }
    
    fn evaluate_expression(&mut self, node: &ASTNode) -> f64 {
        self.evaluate_value(node).to_f64()
    }
    
    fn execute_function_call(&mut self, name: &str, args: &[ASTNode]) {
        if let Some(function) = self.functions.get(name).cloned() {
            if args.len() != function.params.len() {
                eprintln!("{}", error_messages::function_arity_error(name, function.params.len(), args.len()));
                return;
            }
            
            // 先评估所有参数（在当前作用域中）
            let mut arg_values = Vec::new();
            for arg in args.iter() {
                arg_values.push(self.evaluate_value(arg));
            }
            
            // Save current variable scope
            let mut saved_variables = self.variables.clone();
            
            // Remove return value from saved scope to start with a clean state
            saved_variables.remove("__return_value__");
            
            // Create a clean scope with only function parameters
            self.variables.clear();
            for (param, value) in function.params.iter().zip(arg_values.iter()) {
                self.variables.insert(param.clone(), (value.clone(), None));
            }
            
            // Execute function body
            for stmt in function.body.iter() {
                self.evaluate(stmt);
                
                // Check if we have a return value
                if self.variables.contains_key("__return_value__") {
                    break;
                }
            }
            
            // Get return value before restoring scope
            let return_value = self.variables.get("__return_value__").map(|(v, _)| v.clone());
            
            // Restore variable scope
            self.variables = saved_variables;
            
            // Set return value in the restored scope
            if let Some(value) = return_value {
                self.variables.insert("__return_value__".to_string(), (value, None));
            }
        } else {
            eprintln!("{}", error_messages::undefined_function(name));
        }
    }
}