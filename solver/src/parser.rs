use crate::tokenizer::{Token, Operator, Sym};
use crate::expr::{Expr, Prim, Value};

pub fn parse(tokens: &[Token]) -> anyhow::Result<Expr> {
    let (expr, rest) = expr(tokens)?;
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
    let (mut left, mut rest) = op_arith2(tokens)?;
    while !rest.is_empty() {
        match rest {
            [Token::Op(Operator::Plus), rest1 @ ..] => {
                let (right, rest2) = op_arith2(rest1)?;
                left = Expr::Prim(Prim::Add(box left, box right));
                rest = rest2;
            }
            [Token::Op(Operator::Minus), rest1 @ ..] => {
                let (right, rest2) = op_arith2(rest1)?;
                left = Expr::Prim(Prim::Sub(box left, box right));
                rest = rest2;
            } // +, - でなければそこでarith1の式が完了している
            _ => return Ok((left, rest))
        }
    }
    Ok((left, rest))
}
// 結合度が高いもの
// *
fn op_arith2(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    let (mut left, mut rest) = value(tokens)?;
    while !rest.is_empty() {
        match rest {
            [Token::Op(Operator::Mul), rest1 @ ..] => {
                let (right, rest2) = value(rest1)?;
                left = Expr::Prim(Prim::Mul(box left, box right));
                rest = rest2;
            },
            _ => return Ok((left, rest))
        }
    }

    Ok((left, rest))
}

fn value(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    match tokens {
        [Token::Int(i), rest @ ..] => Ok((Expr::Value(Value::Int(*i)),rest)),
        [Token::Bool(i), rest @ ..] => Ok((Expr::Value(Value::Bool(*i)),rest)),
        [Token::Sym(Sym::LParen), rest @ ..] => {
            let (expr, rest1) = expr(rest)?;
            if let [Token::Sym(Sym::RParen), rest2 @ ..] = rest1 {
                Ok((expr, rest2))
            } else {
                Err(anyhow::anyhow!("')' not found"))
            }

        }
        _ => expr(tokens)
    }
}
