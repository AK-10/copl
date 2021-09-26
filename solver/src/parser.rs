use std::fmt;
use std::ops;

use crate::tokenizer::{Token, Operator};

#[derive(Debug, Clone)]
pub enum Value {
    Int(usize),
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
    type Output = usize;
    fn add(self, rhs: Value) -> usize {
        match (self, rhs) {
            (Value::Int(l), Value:: Int(r)) => l + r,
            _ => panic!("type error: + operator cannot int + bool")
        }
    }
}

impl ops::Sub<Value> for Value {
    type Output = usize;
    fn sub(self, rhs: Value) -> usize {
        match (self, rhs) {
            (Value::Int(l), Value:: Int(r)) => l - r,
            _ => panic!("type error: - operator cannot int - bool")
        }
    }
}

impl ops::Mul<Value> for Value {
    type Output = usize;
    fn mul(self, rhs: Value) -> usize {
        match (self, rhs) {
            (Value::Int(l), Value:: Int(r)) => l * r,
            _ => panic!("type error: * operator cannot int * bool")
        }
    }
}

#[derive(Debug)]
pub enum Prim {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    //LessThan(Box<Expr>, Box<Expr>)
}

impl fmt::Display for Prim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Prim::Add(l, r) => write!(f, "{} + {}", l, r),
            Prim::Sub(l, r) => write!(f, "{} - {}", l, r),
            Prim::Mul(l, r) => write!(f, "{} * {}", l, r)
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Value(Value),
    Prim(Prim),
    //IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Expr::Value(x) => write!(f, "{}", x),
            Expr::Prim(x) => write!(f, "{}", x),
        }
    }
}

pub fn parse(tokens: &[Token]) -> anyhow::Result<Expr> {
    let (expr, rest) = expr(tokens)?;
    println!("rest: {:?}", rest);
    match rest {
        [] => Ok(expr),
        _ => Err(anyhow::anyhow!("syntax error"))
    }
}

fn expr(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    op_arith1(tokens)
}

// 結合度が低いもの
// +, -
fn op_arith1(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    let (left, rest) = op_arith2(tokens)?;
    match rest {
        [Token::Op(Operator::Plus), rest @ ..] => {
            let (right, rest) = op_arith1(rest)?;
            Ok((Expr::Prim(Prim::Add(box left, box right)), rest))
        },
        [Token::Op(Operator::Minus), rest @ ..] => {
            let (right, rest) = op_arith1(rest)?;
            Ok((Expr::Prim(Prim::Sub(box left, box right)), rest))
        },
        _ => Ok((left, rest)) // mulの場合にも対応する
    }
}
// 結合度が高いもの
// *
fn op_arith2(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    let (left, rest) = value(tokens)?;
    match rest {
        [Token::Op(Operator::Mul), rest @ ..] => {
            let (right, rest) = op_arith2(rest)?;
            Ok((Expr::Prim(Prim::Mul(box left, box right)), rest))
        },
        _ => Ok((left, rest))
    }
}

fn value(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    match tokens {
        [Token::Int(i), rest @ ..] => Ok((Expr::Value(Value::Int(*i)),rest)),
        [Token::Bool(i), rest @ ..] => Ok((Expr::Value(Value::Bool(*i)),rest)),
        _ => expr(tokens)
    }
}
