use crate::expr::{Expr, Prim, Value, Unary};

use std::fmt;
use std::ops;

// BNF
// EvalML1
// i ∈ int
// b ∈ {true, false}
// v ∈ Value ::= i | b
// r ∈ Res ::= v | error
// e ∈ Exp ::= i | b | e op e | if e then e else e | (e)
// op ∈ Prim ::= + | - | * | <

// const OPERATORS: [&str; 4] = [
//     "+","-", "*", "<"
// ];

#[derive(Debug)]
pub enum EvalResult {
    Value(Value),
    Err(EvalError)
}

impl fmt::Display for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalResult::Value(v) => write!(f, "{}", v),
            EvalResult::Err(_) => write!(f, "error")
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    IfInt,
    PlusBoolL,
    PlusBoolR,
    MinusBoolL,
    MinusBoolR,
    TimesBoolL,
    TimesBoolR,
    LtBoolL,
    LtBoolR,
    IfError,
    IfTError,
    IfFError,
    PlusErrorL,
    PlusErrorR,
    MinusErrorL,
    MinusErrorR,
    TimesErrorL,
    TimesErrorR,
    LtErrorL,
    LtErrorR,
    UnaryMinusBool,
    Unknown
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            EvalError::IfInt => write!(f, "E-IfInt"),
            EvalError::PlusBoolL => write!(f, "E-PlusBoolL"),
            EvalError::PlusBoolR => write!(f, "E-PlusBoolR"),
            EvalError::MinusBoolL => write!(f, "E-MinusBoolL"),
            EvalError::MinusBoolR => write!(f, "E-MinusBoolR"),
            EvalError::TimesBoolL => write!(f, "E-TimesBoolL"),
            EvalError::TimesBoolR => write!(f, "E-TimesBoolR"),
            EvalError::LtBoolL => write!(f, "E-LtBoolL"),
            EvalError::LtBoolR => write!(f, "E-LtBoolR"),
            EvalError::IfError => write!(f, "E-IfError"),
            EvalError::IfTError => write!(f, "E-IfTError"),
            EvalError::IfFError => write!(f, "E-IfFError"),
            EvalError::PlusErrorL => write!(f, "E-PlusErrorL"),
            EvalError::PlusErrorR => write!(f, "E-PlusErrorR"),
            EvalError::MinusErrorL => write!(f, "E-MinusErrorL"),
            EvalError::MinusErrorR => write!(f, "E-MinusErrorR"),
            EvalError::TimesErrorL => write!(f, "E-TimesErrorL"),
            EvalError::TimesErrorR => write!(f, "E-TimesErrorR"),
            EvalError::LtErrorL => write!(f, "E-LtErrorL"),
            EvalError::LtErrorR => write!(f, "E-LtErrorR"),
            EvalError::UnaryMinusBool => write!(f, "E-UnaryMinusBool"),
            EvalError::Unknown => write!(f, "E-PlusBoolL"),
        }
    }
}

impl ops::Add<EvalResult> for EvalResult {
    type Output = EvalResult;
    fn add(self, rhs: EvalResult) -> EvalResult {
        match (self, rhs) {
            (EvalResult::Value(Value::Int(l)), EvalResult::Value(Value::Int(r))) =>
                EvalResult::Value(Value::Int(l + r)),
            (EvalResult::Value(Value::Bool(_)), _) => EvalResult::Err(EvalError::PlusBoolL),
            (_, EvalResult::Value(Value::Bool(_))) => EvalResult::Err(EvalError::PlusBoolR),
            (EvalResult::Err(_), _) => EvalResult::Err(EvalError::PlusErrorL),
            (_, EvalResult::Err(_)) => EvalResult::Err(EvalError::PlusErrorR)
        }
    }
}

impl ops::Sub<EvalResult> for EvalResult {
    type Output = EvalResult;
    fn sub(self, rhs: EvalResult) -> EvalResult {
        match (self, rhs) {
            (EvalResult::Value(Value::Int(l)), EvalResult::Value(Value::Int(r))) =>
                EvalResult::Value(Value::Int(l - r)),
            (EvalResult::Value(Value::Bool(_)), _) => EvalResult::Err(EvalError::MinusBoolL),
            (_, EvalResult::Value(Value::Bool(_))) => EvalResult::Err(EvalError::MinusBoolR),
            (EvalResult::Err(_), _) => EvalResult::Err(EvalError::MinusErrorL),
            (_, EvalResult::Err(_)) => EvalResult::Err(EvalError::MinusErrorR)
       }
    }
}

impl ops::Mul<EvalResult> for EvalResult {
    type Output = EvalResult;
    fn mul(self, rhs: EvalResult) -> EvalResult {
        match (self, rhs) {
            (EvalResult::Value(Value::Int(l)), EvalResult::Value(Value::Int(r))) => EvalResult::Value(Value::Int(l * r)),
            (EvalResult::Value(Value::Bool(_)), _) => EvalResult::Err(EvalError::TimesBoolL),
            (_, EvalResult::Value(Value::Bool(_))) => EvalResult::Err(EvalError::TimesBoolR),
            (EvalResult::Err(_), _) => EvalResult::Err(EvalError::TimesErrorL),
            (_, EvalResult::Err(_)) => EvalResult::Err(EvalError::TimesErrorR)
        }
    }
}

impl ops::Neg for EvalResult {
    type Output = EvalResult;
    fn neg(self) -> EvalResult {
        match self {
            EvalResult::Value(Value::Int(i)) => EvalResult::Value(Value::Int(-i)),
            EvalResult::Value(Value::Bool(_)) => EvalResult::Err(EvalError::UnaryMinusBool),
            e @ EvalResult::Err(_) => e
        }
    }
}

pub fn solve(expr: Expr) {
    apply_rule(&expr);
}

fn eval(expr: &Expr) -> EvalResult {
    match expr {
        Expr::Value(x) => EvalResult::Value(x.clone()),
        Expr::Unary(_) => -eval(expr),
        Expr::Prim(p) => {
            match p {
                Prim::Add(l, r) => eval(l) + eval(r),
                Prim::Sub(l, r) => eval(l) - eval(r),
                Prim::Mul(l, r) => eval(l) * eval(r),
                Prim::LessThan(l, r) => {
                    let l = eval(l);
                    let r = eval(r);
                    match (l, r) {
                        (EvalResult::Value(Value::Int(l)), EvalResult::Value(Value::Int(r))) =>
                            EvalResult::Value(Value::Bool(l < r)),
                        (EvalResult::Value(Value::Bool(_)), _) => EvalResult::Err(EvalError::LtBoolL),
                        (_, EvalResult::Value(Value::Bool(_))) => EvalResult::Err(EvalError::LtBoolR),
                        (EvalResult::Err(_), _) => EvalResult::Err(EvalError::LtErrorL),
                        (_, EvalResult::Err(_)) => EvalResult::Err(EvalError::LtErrorR)
                    }
                },
            }
        }
        Expr::IfThenElse(cond, then, els) => {
            match eval(cond) {
                EvalResult::Value(Value::Bool(b)) => {
                    if b {
                        eval(then)
                    } else {
                        eval(els)
                    }
                }
                EvalResult::Value(Value::Int(_)) => EvalResult::Err(EvalError::IfInt),
                e @ EvalResult::Err(_) => e
            }
        }
    }
}

fn apply_rule(expr: &Expr) {
    match (expr, eval(expr)) {
        (Expr::Value(Value::Int(i)), res) => println!("{} evalto {} by E-Int {{}};", i, res),
        (Expr::Value(Value::Bool(b)), res) => println!("{} evalto {} by E-Bool {{}};", b, res),
        (Expr::Unary(Unary::Minus(_)), res) => println!("{} evalto {} by E-Int {{}};", expr, res),
        (Expr::Prim(Prim::Add(l, r)), EvalResult::Value(v)) => {
            println!("{} evalto {} by E-Plus {{", expr, v);
            apply_rule(l);
            apply_rule(r);
            println!("{} plus {} is {} by B-Plus {{}};", eval(l), eval(r), v);
            println!("}};");
        }
        (Expr::Prim(Prim::Add(l, r)), EvalResult::Err(e)) => {
            println!("{} evalto {} by {} {{", expr, "error", e);
            match e {
                EvalError::PlusBoolL => apply_rule(l),
                EvalError::PlusBoolR => apply_rule(r),
                EvalError::PlusErrorL => apply_rule(l),
                EvalError::PlusErrorR => apply_rule(r),
                _ => unreachable!("internal: unreachable point apply_rule: Add")
            }
            println!("}};");
        }
        (Expr::Prim(Prim::Sub(l, r)), EvalResult::Value(v)) => {
            println!("{} evalto {} by E-Minus {{", expr, v);
            apply_rule(l);
            apply_rule(r);
            println!("{} minus {} is {} by B-Minus {{}};", eval(l), eval(r), v);
            println!("}};");
        }
        (Expr::Prim(Prim::Sub(l, r)), EvalResult::Err(e)) => {
            println!("{} evalto {} by {} {{", expr, "error", e);
            match e {
                EvalError::MinusBoolL => apply_rule(l),
                EvalError::MinusBoolR => apply_rule(r),
                EvalError::MinusErrorL => apply_rule(l),
                EvalError::MinusErrorR => apply_rule(r),
                _ => unreachable!("internal: unreachable point apply_rule: Sub")
            }
            println!("}};");
        }
        (Expr::Prim(Prim::Mul(l, r)), EvalResult::Value(v)) => {
            println!("{} evalto {} by E-Times {{", expr, v);
            apply_rule(l);
            apply_rule(r);
            println!("{} times {} is {} by B-Times {{}};", eval(l), eval(r), v);
            println!("}};");
        }
        (Expr::Prim(Prim::Mul(l, r)), EvalResult::Err(e)) => {
            println!("{} evalto {} by {} {{", expr, "error", e);
            match e {
                EvalError::TimesBoolL => apply_rule(l),
                EvalError::TimesBoolR => apply_rule(r),
                EvalError::TimesErrorL => apply_rule(l),
                EvalError::TimesErrorR => apply_rule(r),
                _ => unreachable!("internal: unreachable point apply_rule: Mul")
            }
            println!("}};");
        }
        (Expr::Prim(Prim::LessThan(l, r)), EvalResult::Value(v)) => {
            println!("{} evalto {} by E-Lt {{", expr, v);
            apply_rule(l);
            apply_rule(r);
            println!("{} times {} is {} by B-Lt {{}};", eval(l), eval(r), v);
            println!("}};");
        }
        (Expr::Prim(Prim::LessThan(l, r)), EvalResult::Err(e)) => {
            println!("{} evalto {} by {} {{", expr, "error", e);
            match e {
                EvalError::LtBoolL => apply_rule(l),
                EvalError::LtBoolR => apply_rule(r),
                EvalError::LtErrorL => apply_rule(l),
                EvalError::LtErrorR => apply_rule(r),
                _ => unreachable!("internal: unreachable point apply_rule: Lt")
            }
            println!("}};");
        }
        (Expr::IfThenElse(cond, then, els), EvalResult::Value(v)) => {
            let cond_result = eval(cond);
            if let EvalResult::Value(Value::Bool(true)) = cond_result {
                println!("{} evalto {} by E-IfT {{", expr, v);
                apply_rule(cond);
                apply_rule(then);
            } else {
                println!("{} evalto {} by E-IfF {{", expr, v);
                apply_rule(cond);
                apply_rule(els);
            }
            println!("}};");
        }
        (Expr::IfThenElse(cond, then, els), EvalResult::Err(e)) => {
            println!("{} evalto {} by {} {{", expr, "error", e);
            match e {
                EvalError::IfError => apply_rule(cond),
                EvalError::IfInt => apply_rule(cond),
                EvalError::IfTError => apply_rule(then),
                EvalError::IfFError => apply_rule(els),
                _ => unreachable!("internal: unreachable point apply_rule: IfThenElse")
            }
            println!("}};");
        }
    }
    //match expr {
    //    Expr::Value(Value::Int(i)) => { println!("{} evalto {} by E-Int {{}};", i, eval(expr)); }
    //    Expr::Value(Value::Bool(b)) => { println!("{} evalto {} by E-Bool {{}};", b, eval(expr)); }
    //    Expr::Unary(Unary::Minus(_)) => { println!("{} evalto {} by E-Int {{}};", expr, eval(expr)); }
    //    Expr::Prim(Prim::Add(l, r)) => {
    //        let result = eval(expr);
    //        println!("{} evalto {} by E-Plus {{", expr, result);
    //        apply_rule(l);
    //        apply_rule(r);
    //        println!("{} plus {} is {} by B-Plus {{}};", eval(l), eval(r), result);
    //        println!("}};");
    //    }
    //    Expr::Prim(Prim::Sub(l, r)) => {
    //        let result = eval(expr);
    //        println!("{} evalto {} by E-Minus {{", expr, result);
    //        apply_rule(l);
    //        apply_rule(r);
    //        println!("{} minus {} is {} by B-Minus {{}};", eval(l), eval(r), result);
    //        println!("}};");
    //    }
    //    Expr::Prim(Prim::Mul(l, r)) => {
    //        let result = eval(expr);
    //        println!("{} evalto {} by E-Times {{", expr, result);
    //        apply_rule(l);
    //        apply_rule(r);
    //        println!("{} times {} is {} by B-Times {{}};", eval(l), eval(r), result);
    //        println!("}};");
    //    }
    //    Expr::Prim(Prim::LessThan(l, r)) => {
    //        let result = eval(expr);
    //        println!("{} evalto {} by E-Lt {{", expr, result);
    //        apply_rule(l);
    //        apply_rule(r);
    //        println!("{} less than {} is {} by B-Lt {{}};", eval(l), eval(r), result);
    //        println!("}};");
    //    }
    //    Expr::IfThenElse(cond, then, els) => {
    //        let result = eval(expr);
    //        let cond_result = eval(cond);
    //        if let EvalResult::Value(Value::Bool(true)) = cond_result {
    //            println!("{} evalto {} by E-IfT {{", expr, result);
    //            apply_rule(cond);
    //            apply_rule(then);
    //        } else {
    //            println!("{} evalto {} by E-IfF {{", expr, result);
    //            apply_rule(cond);
    //            apply_rule(els);
    //        }
    //        println!("}};");
    //    }
    //}
}
