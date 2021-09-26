
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
    Mul(Box<Expr>, Box<Expr>),
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
