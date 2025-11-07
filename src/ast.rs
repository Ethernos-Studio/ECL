#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Str,
    Bool,
    Float,
    Double,
    Any, // 用于尚未确定类型的表达式
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Print(Box<ASTNode>),
    Println(Box<ASTNode>),
    // 变量声明：<type>name = value 或 var <type>name = value
    Var(String, Box<ASTNode>), // name, value
    TypedVar(String, Type, Box<ASTNode>), // name, type, value
    // 新的for循环格式：for i in range(start, end) { body }
    For(String, Box<ASTNode>, Vec<ASTNode>),
    If(Box<ASTNode>, Vec<ASTNode>, Option<Vec<ASTNode>>),
    Assign(String, Box<ASTNode>),
    Identifier(String),
    Number(f64),
    String(String),
    Bool(bool),
    BinaryOp(Box<ASTNode>, String, Box<ASTNode>),
    // 函数定义：func name(param1, param2) { body }
    Function(String, Vec<String>, Vec<ASTNode>),
    // 表达式函数定义：expr name(l a, r b) { body }
    Expr(String, Vec<(String, String)>, Vec<ASTNode>),
    // 函数调用：name(arg1, arg2)
    FunctionCall(String, Vec<ASTNode>),
    // 返回语句：return value
    Return(Box<ASTNode>),
}