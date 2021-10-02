use std::fmt;
use std::ops;
use std::cmp;
use std::cmp::Ordering;

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

impl ops::Add<Value> for Value {
    type Output = isize;
    fn add(self, rhs: Value) -> isize {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => l + r,
            _ => panic!("type error: `+` operator cannot int + bool")
        }
    }
}

impl ops::Sub<Value> for Value {
    type Output = isize;
    fn sub(self, rhs: Value) -> isize {
        match (self, rhs) {
            (Value::Int(l), Value::Int(r)) => l - r,
            _ => panic!("type error: `-` operator cannot apply int - bool")
        }
    }
}

impl ops::Mul<Value> for Value {
    type Output = isize;
    fn mul(self, rhs: Value) -> isize {
        match (self, rhs) {
            (Value::Int(l), Value:: Int(r)) => l * r,
            _ => panic!("type error: `*` operator cannot apply int * bool")
        }
    }
}

impl cmp::PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Int(l), Value::Int(r)) => *l == *r,
            (Value::Bool(l), Value::Bool(r)) => *l == *r,
            _ => panic!("type error: `==` operator cannot apply int * bool")
        }
    }
}

impl cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(l), Value::Int(r)) => l.partial_cmp(r),
            (Value::Bool(l), Value::Bool(r)) => l.partial_cmp(r),
            _ => panic!("type error: < operator cannot apply int * bool")
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
