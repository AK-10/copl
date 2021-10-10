use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Int(isize),
    Bool(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Bool(b) => write!(f, "{}", b)
        }
    }
}

#[derive(Debug)]
pub enum Unary {
    Minus(Box<Expr>)
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Unary::Minus(i) => write!(f, "-{}", i),
        }
    }
}

#[derive(Debug)]
pub enum Prim {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    LessThan(Box<Expr>, Box<Expr>)
}

impl fmt::Display for Prim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Prim::Add(l, r) => write!(f, "{} + {}", l, r),
            Prim::Sub(l, r) => write!(f, "{} - {}", l, r),
            Prim::Mul(l, r) => write!(f, "{} * {}", l, r),
            Prim::LessThan(l, r) => write!(f, "{} < {}", l, r)
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Value(Value),
    Unary(Unary),
    Prim(Prim),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Expr::Value(x) => write!(f, "{}", x),
            Expr::Unary(x) => write!(f, "{}", x),
            Expr::Prim(x) => write!(f, "{}", x),
            Expr::IfThenElse(cond, then, els) => write!(f, "if {} then {} else {}", *cond, *then, *els)
        }
    }
}
