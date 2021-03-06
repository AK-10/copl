use crate::expr::{Expr, Prim, Value, Unary, EnvVar, Env, Form};

use std::fmt;
use std::ops;

// BNF
// EvalML1
// i ∈ int
// b ∈ {true, false}
// x, y ∈ Var
// v ∈ Value ::= i | b
// ε ∈ ∅ | ε , x = v
// r ∈ Res ::= v | error
// e ∈ Exp ::= i | b | e op e | if e then e else e | (e) | let x = e in e
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
    LetError1,
    LetError2,
    VarErr
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
            EvalError::LetError1 => write!(f, "E-LetError1"),
            EvalError::LetError2 => write!(f, "E-LetError2"),
            EvalError::VarErr => write!(f, "E-VarErr")
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

pub fn solve(form: &Form) {
    let env = &form.0;
    let expr = &form.1;
    apply_rule(env, expr);
}

fn eval(env: &Env, expr: &Expr) -> EvalResult {
    match expr {
        Expr::Value(x) => EvalResult::Value(x.clone()),
        Expr::Unary(_) => -eval(env, expr),
        Expr::Ident(name) => {
            match get_env_var(env, name) {
                Some(v) => EvalResult::Value(v.clone()),
                None => EvalResult::Err(EvalError::VarErr) // env var not found
            }
        }
        Expr::Prim(p) => {
            match p {
                Prim::Add(l, r) => eval(env, l) + eval(env, r),
                Prim::Sub(l, r) => eval(env, l) - eval(env, r),
                Prim::Mul(l, r) => eval(env, l) * eval(env, r),
                Prim::LessThan(l, r) => {
                    let l = eval(env, l);
                    let r = eval(env, r);
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
            match eval(env, cond) {
                EvalResult::Value(Value::Int(_)) => EvalResult::Err(EvalError::IfInt),
                EvalResult::Value(Value::Bool(b)) => {
                    if b {
                        match eval(env, then) {
                            EvalResult::Err(_) => EvalResult::Err(EvalError::IfTError),
                            res => res
                        }
                    } else {
                        match eval(env, els) {
                            EvalResult::Err(_) => EvalResult::Err(EvalError::IfTError),
                            res => res
                        }
                    }
                }
                e @ EvalResult::Err(_) => e
            }
        }
        Expr::Let(var, var_exp, expr) => {
            let var_exp_evaled = eval(env, var_exp);
            match var_exp_evaled {
                EvalResult::Value(v) => {
                    let val = Expr::Value(v);
                    let new_env = Env(vec![EnvVar(var, box val)]);
                    if let result @ EvalResult::Value(_) = eval(&new_env.appended(env), expr) {
                        result
                    } else {
                        EvalResult::Err(EvalError::LetError2)
                    }
                }
                EvalResult::Err(_) => {
                    EvalResult::Err(EvalError::LetError1)
                }
            }
        }
    }
}

fn apply_rule(env: &Env, expr: &Expr) {
    let evaled = eval(env, expr);
    match expr {
        Expr::Value(Value::Int(i)) => println!("{} {} evalto {} by E-Int {{}};", env.form(), i, i),
        Expr::Value(Value::Bool(b)) => println!("{} {} evalto {} by E-Bool {{}};", env.form(), b, b),
        Expr::Unary(Unary::Minus(_)) => println!("{} {} evalto {} by E-Int {{}};", env.form(), expr, evaled),
        Expr::Prim(Prim::Add(l, r)) => {
            let evaled = eval(env, expr);
            println!("{} {} evalto {} by E-Plus {{", env.form(), expr, evaled);
            match evaled {
                EvalResult::Value(v) => {
                    apply_rule(env, l);
                    apply_rule(env, r);
                    println!("{} plus {} is {} by B-Plus {{}};", eval(env, l), eval(env, r), v);
                }
                EvalResult::Err(EvalError::PlusBoolL) => apply_rule(env, l),
                EvalResult::Err(EvalError::PlusBoolR) => apply_rule(env, r),
                EvalResult::Err(EvalError::PlusErrorL) => apply_rule(env, l),
                EvalResult::Err(EvalError::PlusErrorR) => apply_rule(env, r),
                _ => unreachable!("internal: unreachable point apply_rule: Add")
            }
            println!("}};");
        }
        Expr::Prim(Prim::Sub(l, r)) => {
            println!("{} {} evalto {} by E-Minus {{", env.form(), expr, evaled);
                match evaled {
                EvalResult::Value(v) => {
                    apply_rule(env, l);
                    apply_rule(env, r);
                    println!("{} minus {} is {} by B-Minus {{}};", eval(env, l), eval(env, r), v);
                }
                EvalResult::Err(EvalError::MinusBoolL) => apply_rule(env, l),
                EvalResult::Err(EvalError::MinusBoolR) => apply_rule(env, r),
                EvalResult::Err(EvalError::MinusErrorL) => apply_rule(env, l),
                EvalResult::Err(EvalError::MinusErrorR) => apply_rule(env, r),
                _ => unreachable!("internal: unreachable point apply_rule: Minus")
            }
            println!("}};");
        }
        Expr::Prim(Prim::Mul(l, r)) => {
            println!("{} {} evalto {} by E-Times {{", env.form(), expr, evaled);
            match evaled {
                EvalResult::Value(v) => {
                    apply_rule(env, l);
                    apply_rule(env, r);
                    println!("{} times {} is {} by B-Times {{}};", eval(env, l), eval(env, r), v);
                }
                EvalResult::Err(EvalError::TimesBoolL) => apply_rule(env, l),
                EvalResult::Err(EvalError::TimesBoolR) => apply_rule(env, r),
                EvalResult::Err(EvalError::TimesErrorL) => apply_rule(env, l),
                EvalResult::Err(EvalError::TimesErrorR) => apply_rule(env, r),
                _ => unreachable!("internal: unreachable point apply_rule: Times")
            }
            println!("}};");
        }
        Expr::Prim(Prim::LessThan(l, r)) => {
            println!("{} {} evalto {} by E-Lt {{", env.form(), expr, evaled);
            match evaled {
                EvalResult::Value(v) => {
                    apply_rule(env, l);
                    apply_rule(env, r);
                    println!("{} less than {} is {} by B-Lt {{}};", eval(env, l), eval(env, r), v);
                }
                EvalResult::Err(EvalError::LtBoolL) => apply_rule(env, l),
                EvalResult::Err(EvalError::LtBoolR) => apply_rule(env, r),
                EvalResult::Err(EvalError::LtErrorL) => apply_rule(env, l),
                EvalResult::Err(EvalError::LtErrorR) => apply_rule(env, r),
                _ => unreachable!("internal: unreachable point apply_rule: Lt")
            }
            println!("}};");
        }
        Expr::IfThenElse(cond, then, els) => {
            match evaled {
                EvalResult::Value(v) => {
                    let cond_result = eval(env, cond);
                    if let EvalResult::Value(Value::Bool(true)) = cond_result {
                        println!("{} {} evalto {} by E-IfT {{", env.form(), expr, v);
                        apply_rule(env, cond);
                        apply_rule(env, then);
                    } else {
                        println!("{} {} evalto {} by E-IfF {{", env.form(), expr, v);
                        apply_rule(env, cond);
                        apply_rule(env, els);
                    }
                }
                EvalResult::Err(EvalError::IfError) => apply_rule(env, cond),
                EvalResult::Err(EvalError::IfInt) => {
                    println!("{} {} evalto {} by E-IfInt {{", env.form(), expr, evaled);
                    apply_rule(env, cond);
                },
                EvalResult::Err(EvalError::IfTError) => {
                    println!("{} {} evalto {} by E-IfT {{", env.form(), expr, evaled);
                    apply_rule(env, cond);
                    apply_rule(env, then);
                }
                EvalResult::Err(EvalError::IfFError) => {
                    println!("{} {} evalto {} by E-IfF {{", env.form(), expr, evaled);
                    apply_rule(env, cond);
                    apply_rule(env, els);
                }
                _ => unreachable!("internal: unreachable point apply_rule: IfThenElse")
            }

            println!("}};");
        }
        Expr::Ident(name) => {
            match evaled {
                EvalResult::Value(v) => {
                    match env.0.first() {
                        Some(EnvVar(n, _)) => {
                            if n == name {
                                println!("{} {} evalto {} by E-Var1 {{}};", env.form(), name, v);
                            } else {
                                println!("{} {} evalto {} by E-Var2 {{", env.form(), name, v);
                                apply_rule(&Env(env.0[1..env.0.len()].to_vec()), expr);
                                println!("}};");
                            }
                        }
                        None => unreachable!("solver Ident: unreachable")
                    }
                }
                _ => unimplemented!("unimplemented ident errors")
            }
        }
        Expr::Let(var, var_exp, exp) => {
            println!("{} {} evalto {} by E-Let {{", env.form(), expr, evaled);
            match evaled {
                EvalResult::Value(_) => {
                    apply_rule(env, var_exp);
                    let var_exp_evaled = eval(env, var_exp);
                    if let EvalResult::Value(v) = var_exp_evaled {
                        let val = Expr::Value(v);
                        let new_env = Env(vec![EnvVar(var, box val)]);
                        let new_env = new_env.appended(env);
                        apply_rule(&new_env, exp)
                    } else {
                        println!("unexpected var_exp")
                    }
                }
                _ => unimplemented!("unimplemented let errors")
            }
            println!("}};");
        }
    }
}

fn get_env_var<'a>(env: &'a Env, name: &'a String) -> Option<&'a Value> {
    for e in &env.0 {
        if e.0 == name {
            if let Expr::Value(x) = e.1.as_ref() {
                return Some(x)
            }
        }
    }

    None
}
