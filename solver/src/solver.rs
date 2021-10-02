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
                Prim::LessThan(l, r) => Value::Bool(eval(l) < eval(r)),
            }
        }
        Expr::IfThenElse(cond, then, els) => {
            if let Value::Bool(b) = eval(cond) {
                if b {
                    eval(then)
                } else {
                    eval(els)
                }
            } else {
                panic!("condition must be bool")
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
            println!("}};");
        }
        Expr::Prim(Prim::Sub(l, r)) => {
            let result = eval(expr);
            println!("{} evalto {} by E-Minus {{", expr, result);
            apply_rule(l);
            apply_rule(r);
            println!("{} minus {} is {} by B-Minus {{}};", eval(l), eval(r), result);
            println!("}};");
        }
        Expr::Prim(Prim::Mul(l, r)) => {
            let result = eval(expr);
            println!("{} evalto {} by E-Times {{", expr, result);
            apply_rule(l);
            apply_rule(r);
            println!("{} times {} is {} by B-Times {{}};", eval(l), eval(r), result);
            println!("}};");
        }
        Expr::Prim(Prim::LessThan(l, r)) => {
            let result = eval(expr);
            println!("{} evalto {} by E-Lt {{", expr, result);
            apply_rule(l);
            apply_rule(r);
            println!("{} less than {} is {} by B-Lt {{}};", eval(l), eval(r), result);
            println!("}};");
        }
        Expr::IfThenElse(cond, then, els) => {
            let result = eval(expr);
            let cond_result = eval(cond);
            if let Value::Bool(true) = cond_result {
                println!("{} evalto {} by E-IfT {{", expr, result);
                apply_rule(cond);
                apply_rule(then);
            } else {
                println!("{} evalto {} by E-IfF {{", expr, result);
                apply_rule(cond);
                apply_rule(els);
            }
            println!("}};");
        }
    }
}
