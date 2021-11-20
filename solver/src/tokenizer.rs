use std::str;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    LessThan,
    Equal,
}

#[derive(Debug, PartialEq)]
pub enum Sym {
    LParen,
    RParen,
    Comma
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Int(isize),
    Bool(bool),
    Op(Operator),
    Sym(Sym),
    If,
    Then,
    Else,
    Var(String),
    Let,
    In
}

pub fn tokenize<'a>(chars: &'a [u8]) -> anyhow::Result<Vec<Token>> {
    match chars {
        [b'1'..=b'9', ..] => {
            let (num, rest) = get_num(chars);
            Ok(new_token(Token::Int(num), rest)?)
        }
        [b'i', b'f', rest @ ..] => {
            Ok(new_token(Token::If, rest)?)
        }
        [b't', b'h', b'e', b'n', rest @ ..] => {
            Ok(new_token(Token::Then, rest)?)
        }
        [b'e', b'l', b's', b'e', rest @ ..] => {
            Ok(new_token(Token::Else, rest)?) }
        [b't', b'r', b'u', b'e', rest @ ..] => {
            Ok(new_token(Token::Bool(true), rest)?)
        }
        [b'f', b'a', b'l', b's', b'e', rest @ ..] => {
            Ok(new_token(Token::Bool(false), rest)?)
        }
        [b'+', rest @ ..] => {
            Ok(new_token(Token::Op(Operator::Plus), rest)?)
        }
        [b'-', rest @ ..] => {
            Ok(new_token(Token::Op(Operator::Minus), rest)?)
        }
        [b'*', rest @ ..] => {
            Ok(new_token(Token::Op(Operator::Mul), rest)?)
        }
        [b'<', rest @ ..] => {
            Ok(new_token(Token::Op(Operator::LessThan), rest)?)
        }
        [b'(', rest @ ..] => {
            Ok(new_token(Token::Sym(Sym::LParen), rest)?)
        }
        [b')', rest @ ..] => {
            Ok(new_token(Token::Sym(Sym::RParen), rest)?)
        }
        [b'=', rest @ ..] => {
            Ok(new_token(Token::Op(Operator::Equal), rest)?)
        }
        [b',', rest @ ..] => {
            Ok(new_token(Token::Sym(Sym::Comma), rest)?)
        }
        [b'l', b'e', b't', rest @ ..] => {
            Ok(new_token(Token::Let, rest)?)
        }
        [b'i', b'n', rest @ ..] => {
            Ok(new_token(Token::In, rest)?)
        }
        [b'_' | b'a'..=b'z', ..] => {
            let (var, rest) = get_var(chars);
            Ok(new_token(Token::Var(var), rest)?)
        }

        [first, rest @ ..] if first.is_ascii_whitespace() => tokenize(rest),
        [] => Ok(Vec::new()),
        x => Err(anyhow::anyhow!("unexpected token: {:?}", str::from_utf8(x).unwrap()))
    }
}

fn new_token(token: Token, chars: &[u8]) -> anyhow::Result<Vec<Token>> {
    let mut tokens = vec![token];
    tokens.append(&mut tokenize(chars)?);

    Ok(tokens)
}

fn get_num(chars: &[u8]) -> (isize, &[u8]) {
    let (num_str, rest) = get_num_str(chars);
    let num = str::from_utf8(&num_str)
        .expect("invalid utf8 number")
        .to_string()
        .parse::<isize>()
        .unwrap();

    (num, rest)
}

fn get_var(chars: &[u8]) -> (String, &[u8]) {
    let (var_str, rest) = get_var_str(chars);

    (String::from_utf8(var_str).expect("invalid utf8 var character"), rest)
}

fn get_var_str(chars: &[u8]) -> (Vec<u8>, &[u8]) {
    match chars {
        [first @ (b'_' | b'a'..=b'z' | b'A'..=b'Z' | b'\''), rest @ ..] => {
            let mut var =vec![*first];
            let (mut rest_var, rest) = get_var_str(rest);
            var.append(&mut rest_var);
            (var, rest)
        }
        _ => (Vec::new(), chars)
    }
}

fn get_num_str(chars: &[u8]) -> (Vec<u8>, &[u8]) {
    match chars {
        [first @ b'0'..=b'9', rest @ ..] => {
            let mut num = vec![*first];
            let (mut rest_num, rest) = get_num_str(rest);
            num.append(&mut rest_num);

            (num, rest)
        }
        _ => (Vec::new(), chars)
    }
}
