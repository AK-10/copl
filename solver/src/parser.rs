
use crate::tokenizer::{Token, Operator};

#[derive(Debug)]
pub enum Value {
    Int(usize),
    Bool(bool),
}

#[derive(Debug)]
pub enum Prim {
    Add(Box<Expr>, Box<Expr>),
    //Sub(Box<Expr>, Box<Expr>),
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
    let (expr, rest) = op(tokens)?;
    match rest {
        [] => Ok(expr),
        _ => Err(anyhow::anyhow!("syntax error"))
    }

}
fn op(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    let (exp, rest) = value(tokens)?;
    match rest {
        [Token::Op(Operator::Plus), rest @ ..] => {
            let (right, rest) = value(rest)?;
            Ok((Expr::Prim(Prim::Add(box exp, box right)), rest))
        }
        [] => Err(anyhow::anyhow!("syntax error")),
        _ => unimplemented!("")
    }
}

fn value(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    match tokens {
        [Token::Int(i), rest @ ..] => Ok((Expr::Value(Value::Int(*i)),rest)),
        _ => unimplemented!()
    }
}
