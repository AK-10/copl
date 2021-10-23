use std::{fmt, fs::write};

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
pub enum Unary<'a> {
    Minus(Box<Expr<'a>>)
}

impl<'a> fmt::Display for Unary<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Unary::Minus(i) => write!(f, "-{}", i),
        }
    }
}

#[derive(Debug)]
pub enum Prim<'a> {
    Add(Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Box<Expr<'a>>, Box<Expr<'a>>),
    LessThan(Box<Expr<'a>>, Box<Expr<'a>>)
}

impl<'a> fmt::Display for Prim<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Prim::Add(l, r) => write!(f, "{} + {}", l, r),
            Prim::Sub(l, r) => write!(f, "{} - {}", l, r),
            Prim::Mul(l, r) => write!(f, "{} * {}", l, r),
            Prim::LessThan(l, r) => write!(f, "{} < {}", l, r)
        }
    }
}

// example
// x = 3, y = 2 |- x
// Form(
//   Env(Var(y, 2), Env(Var(x, 3), Empty)),
//   Var(x)
//   Value(Int(3))
// )

#[derive(Debug)]
pub struct Form<'a>(pub Env<'a>, pub Expr<'a>, pub Expr<'a>);

#[derive(Debug)]
pub struct EnvVar<'a>(pub &'a String, pub Box<Expr<'a>>);

#[derive(Debug)]
pub enum Env<'a> {
    Empty,
    Some(EnvVar<'a>, Box<Env<'a>>)
}

#[derive(Debug)]
pub enum Expr<'a> {
    Value(Value),
    Unary(Unary<'a>),
    Prim(Prim<'a>),
    IfThenElse(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),
    Ident(&'a String)
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Expr::Value(x) => write!(f, "{}", x),
            Expr::Unary(x) => write!(f, "{}", x),
            Expr::Prim(x) => write!(f, "{}", x),
            Expr::IfThenElse(cond, then, els) => write!(f, "if {} then {} else {}", *cond, *then, *els),
            Expr::Ident(name) => write!(f, "{}", name)
        }
    }
}
