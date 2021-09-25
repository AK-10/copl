
use crate::tokenizer::{Token, Operator};

#[derive(Debug)]
pub enum Value {
    Int(usize),
    Bool(bool),
}

#[derive(Debug)]
pub enum Prim {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    //Mul(Box<Expr>, Box<Expr>),
    //LessThan(Box<Expr>, Box<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Value(Value),
    Prim(Prim),
    //IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
}

pub fn parse(tokens: &[Token]) -> anyhow::Result<Expr> {
    let (expr, rest) = expr(tokens)?;
    match rest {
        [] => Ok(expr),
        _ => Err(anyhow::anyhow!("syntax error"))
    }
}

fn expr(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    op1(tokens)
}

// 結合度が低いもの
// +, -
fn op1(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    let (left, rest) = op2(tokens)?;
    match rest {
        [Token::Op(Operator::Plus), rest @ ..] => {
            let (right, rest) = op1(rest)?;
            Ok((Expr::Prim(Prim::Add(box left, box right)), rest))
        },
        [Token::Op(Operator::Minus), rest @ ..] => {
            let (right, rest) = op1(rest)?;
            Ok((Expr::Prim(Prim::Sub(box left, box right)), rest))
        },
        _ => Ok((left, rest))
    }



}
// 結合度が高いもの
// +, -, <
fn op2(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    let (left, rest) = value(tokens)?;
    match tokens {
        [Token::Op(Operator::Mul), rest @ ..] => unimplemented!(""),
        _ => Ok((left, rest))
    }
}

fn value(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    match tokens {
        [Token::Int(i), rest @ ..] => Ok((Expr::Value(Value::Int(*i)),rest)),
        [Token::Bool(i), rest @ ..] => Ok((Expr::Value(Value::Bool(*i)),rest)),
        _ => op1(tokens)
    }
}
