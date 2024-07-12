#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Boolean(bool),
    String(String),
    Assign(Box<Expr>, Box<Expr>),
    VarDeclaration(String, String),
    VarDeclarationWithAssignment(String, String, Box<Expr>),
    Semicolon(String),
    FuncCall(String, Vec<Expr>),
    FuncDef(String, String, Vec<(String, String)>, Box<Expr>),
    MainFuncDef(Box<Expr>),
    Statements(Vec<Expr>),
    Return(Box<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Print(Box<Expr>),
    PrintStr(String),
    Program(Vec<Expr>),
    While(Box<Expr>, Box<Expr>),
}



#[derive(Debug, Clone, PartialEq,Copy)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    LessThanEqual,
    Equal,
    GreaterThanEqual,
    NotEqual,
}


