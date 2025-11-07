#[derive(Debug)]
pub enum ASTNode {
    Print(Box<ASTNode>),
    Println(Box<ASTNode>),
    Var(String, Box<ASTNode>),
    // 新的for循环格式：for i in range(start, end) { body }
    For(String, Box<ASTNode>, Vec<ASTNode>),
    If(Box<ASTNode>, Vec<ASTNode>, Option<Vec<ASTNode>>),
    Assign(String, Box<ASTNode>),
    Identifier(String),
    Number(f64),
    String(String),
    BinaryOp(Box<ASTNode>, String, Box<ASTNode>),
}