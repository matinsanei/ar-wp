use crate::val::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Concat,
    Eq,
    Identical,
    NotEq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOpKind {
    Not,
    Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Value),
    Var(String),
    ArrayLiteral(Vec<(Option<Expr>, Expr)>),
    ArrayAccess {
        base: Box<Expr>,
        dim: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    DynamicCall {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOpKind,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Assign {
        var: String,
        expr: Expr,
    },
    ArrayDimAssign {
        base: Box<Expr>,
        dim: Option<Box<Expr>>,
        expr: Expr,
    },
    Global(Vec<String>),
    FunctionDef {
        name: String,
        params: Vec<(String, Option<Expr>)>,
        body: Vec<Stmt>,
    },
    If {
        cond: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    },
    Foreach {
        expr: Expr,
        key_var: Option<String>,
        val_var: String,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Echo(Vec<Expr>),
    Expr(Expr),
}
