use std::fmt::{self, Write};

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct EnvVar<'a>(pub &'a String, pub Box<Expr<'a>>);

#[derive(Debug, Clone)]
pub struct Env<'a>(pub Vec<EnvVar<'a>>);

impl<'a> Env<'a> {
    pub fn form(&self) -> String {
        let mut buf = String::new();
        for (i, e) in self.0.iter().enumerate().rev() {
            write!(buf, "{} = {}", e.0, e.1).unwrap();
            if i != 0 {
                write!(buf, ", ").unwrap();
            }
        }
        write!(buf, " |-").unwrap();

        buf
    }

    pub fn form_while(&self, name: &'a String) -> String {
        let mut buf = String::new();
        for (i, e) in self.0.iter().enumerate().rev() {
            write!(buf, "{} = {}", e.0, e.1).unwrap();
            if e.0 == name {
                write!(buf, " |-").unwrap();
                return buf
            }
            if i != 0 {
                write!(buf, ", ").unwrap();
            }
        }
        write!(buf, " |-").unwrap();
        buf
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Value(Value),
    Unary(Unary<'a>),
    Prim(Prim<'a>),
    IfThenElse(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),
    Ident(&'a String),
    Let(Env<'a>, Box<Expr<'a>>)
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Expr::Value(x) => write!(f, "{}", x),
            Expr::Unary(x) => write!(f, "{}", x),
            Expr::Prim(x) => write!(f, "{}", x),
            Expr::IfThenElse(cond, then, els) => write!(f, "if {} then {} else {}", *cond, *then, *els),
            Expr::Ident(name) => write!(f, "{}", name),
            Expr::Let(env, expr) => write!(f, "{} {}", env.form(), expr)
        }
    }
}
