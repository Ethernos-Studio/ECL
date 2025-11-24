#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Str,
    Bool,
    Float,
    Double,
    Any, // 用于尚未确定类型的表达式
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Str => "str".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Float => "float".to_string(),
            Type::Double => "double".to_string(),
            Type::Any => "any".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone)]
pub enum ASTNode {
    Print(Box<ASTNode>),
    Println(Box<ASTNode>),
    // 变量声明：<type>name = value 或 var <type>name = value
    Var(String, Box<ASTNode>), // name, value
    TypedVar(String, Type, Box<ASTNode>), // name, type, value
    // 数组声明：var <int>name[size] = {init_values}
    ArrayDecl(String, Type, usize, Vec<ASTNode>), // name, type, size, init_values
    // 列表声明：var name = []
    ListDecl(String, Vec<ASTNode>), // name, init_values
    // 数组/列表索引访问：name[index]
    IndexAccess(Box<ASTNode>, Box<ASTNode>, Position), // array/list expression, index expression, position
    // 数组/列表索引赋值：name[index] = value
    IndexAssign(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>, Position), // array/list expression, index expression, value expression, position
    // 新的for循环格式：for i in range(start, end) { body }
    For(String, Box<ASTNode>, Vec<ASTNode>),
    // while循环：while (condition) { body }
    While(Box<ASTNode>, Vec<ASTNode>),
    If(Box<ASTNode>, Vec<ASTNode>, Option<Vec<ASTNode>>),
    // input语句：input prompt, variable
    Input(Box<ASTNode>, String),
    Assign(String, Box<ASTNode>),
    Identifier(String, Position),
    Number(f64),
    String(String),
    Bool(bool),
    BinaryOp(Box<ASTNode>, String, Box<ASTNode>, Position), // left operand, operator, right operand, position
    // 函数定义：func name(param1, param2) { body }
    Function(String, Vec<String>, Vec<ASTNode>),
    // 表达式函数定义：expr name(l a, r b) { body }
    Expr(String, Vec<(String, String)>, Vec<ASTNode>),
    // 函数调用：name(arg1, arg2)
    FunctionCall(String, Vec<ASTNode>),
    // 返回语句：return value
    Return(Box<ASTNode>),
    // if表达式：if (condition) then_expr else else_expr

    IfExpr(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>),

    // 类型转换：<type>value

    TypeConversion(Type, Box<ASTNode>, Position),
    // import语句：import "filename"
    Import(String, Position),
}