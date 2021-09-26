use crate::parser::{Expr, Prim, Value};

//(1 + 2 evalto 3)

pub fn solve(expr: Expr) {
    println!("{} evalto {}" , expr, eval(&expr));
}

fn eval<'a>(expr: &'a Expr) -> Value {
    match expr {
        Expr::Value(x) => x.clone(),
        Expr::Prim(p) => {
            match p {
                Prim::Add(l, r) => Value::Int(eval(l) + eval(r)),
                Prim::Sub(l, r) => Value::Int(eval(l) - eval(r)),
                Prim::Mul(l, r) => Value::Int(eval(l) * eval(r)),
            }
        }
    }
}
