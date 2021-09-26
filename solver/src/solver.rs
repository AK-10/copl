use crate::expr::{Expr, Prim, Value};

pub fn solve(expr: Expr) {
    apply_rule(&expr);
}

fn eval(expr: &Expr) -> Value {
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

fn apply_rule(expr: &Expr) {
    match expr {
        Expr::Value(Value::Int(i)) => { println!("{} evalto {} by E-Int {{}};", i, eval(expr)); }
        Expr::Value(Value::Bool(b)) => { println!("{} evalto {} by E-Int {{}};", b, eval(expr)); }
        Expr::Prim(Prim::Add(l, r)) => {
            let result = eval(expr);
            println!("{} evalto {} by E-Plus {{", expr, result);
            apply_rule(l);
            apply_rule(r);
            println!("{} plus {} is {} by B-Plus {{}};", eval(l), eval(r), result);
            println!("}}");
        }
        Expr::Prim(Prim::Sub(l, r)) => {
            let result = eval(expr);
            println!("{} evalto {} by E-Minus {{", expr, result);
            apply_rule(l);
            apply_rule(r);
            println!("{} minus {} is {} by B-Minus {{}};", eval(l), eval(r), result);
            println!("}};");
        }
        _ => unimplemented!("prim mul not implemented")

    }
}
