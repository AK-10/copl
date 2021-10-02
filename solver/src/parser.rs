use crate::tokenizer::{Token, Operator, Sym};
use crate::expr::{Expr, Prim, Value, Unary};

pub fn parse(tokens: &[Token]) -> anyhow::Result<Expr> {
    let (expr, rest) = expr(tokens)?;
    match rest {
        [] => Ok(expr),
        _ => Err(anyhow::anyhow!("syntax error"))
    }
}

fn expr(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    op_compare(tokens)
}

fn op_compare(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    let (mut left, mut rest) = op_arith1(tokens)?;
    while !rest.is_empty() {
        match rest {
            [Token::Op(Operator::LessThan), rest1 @ ..] => {
                let (right, rest2) = op_compare(rest1)?;
                left = Expr::Prim(Prim::LessThan(box left, box right));
                rest = rest2;
            }
            _ => return Ok((left, rest))
        }
    }
    Ok((left, rest))
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
    let (mut left, mut rest) = unary(tokens)?;
    while !rest.is_empty() {
        match rest {
            [Token::Op(Operator::Mul), rest1 @ ..] => {
                let (right, rest2) = unary(rest1)?;
                left = Expr::Prim(Prim::Mul(box left, box right));
                rest = rest2;
            },
            _ => return Ok((left, rest))
        }
    }

    Ok((left, rest))
}

fn unary(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    match tokens {
        [Token::Op(Operator::Minus), rest @ ..] => {
            let (value, rest1) = value(rest)?;
            Ok((Expr::Unary(Unary::Minus(box value)), rest1))
        }
        _ => value(tokens)
    }
}

fn value(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    match tokens {
        [Token::Int(i), rest @ ..] => Ok((Expr::Value(Value::Int(*i)),rest)),
        [Token::Bool(i), rest @ ..] => Ok((Expr::Value(Value::Bool(*i)),rest)),
        [Token::Sym(Sym::LParen), ..] => paren_expr(tokens),
        [Token::If, ..] => if_then_else(tokens),
        _ => expr(tokens)
    }
}

fn paren_expr(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    match tokens {
        [Token::Sym(Sym::LParen), rest @ ..] => {
            let (expr, rest1) = expr(rest)?;
            if let [Token::Sym(Sym::RParen), rest2 @ ..] = rest1 {
                Ok((expr, rest2))
            } else {
                Err(anyhow::anyhow!("')' not found"))
            }
        }
        _ => Err(anyhow::anyhow!("internal: unexpected invoke paren_expr"))
    }
}

fn if_then_else(tokens: &[Token]) -> anyhow::Result<(Expr, &[Token])> {
    match tokens {
        [Token::If, rest @ ..] => {
            let (cond, rest1) = expr(rest)?;
            if let [Token::Then, rest2 @ ..] = rest1 {
                let (then, rest3) = expr(rest2)?;
                if let [Token::Else, rest4 @ ..] = rest3 {
                    let (els, rest5) = expr(rest4)?;
                    Ok((Expr::IfThenElse(box cond, box then, box els), rest5))
                } else {
                    Err(anyhow::anyhow!("else section not found"))
                }
            } else {
                Err(anyhow::anyhow!("then section not found"))
            }
        }
        _ => Err(anyhow::anyhow!("internal: unexpected invoke if_then_else"))
    }
}
