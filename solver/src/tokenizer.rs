use std::str;

// BNF
// EvalML1
// i ∈ int
// b ∈ {true, false}
// v ∈ Value ::= i | b
// e ∈ Exp ::= i | b | e op e | if e then e else e | (e)
// op ∈ Prim ::= + | - | * | <

// const OPERATORS: [&str; 4] = [
//     "+","-", "*", "<"
// ];
#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    LessThan
}

#[derive(Debug)]
pub enum Sym {
    LParen,
    RParen
}

#[derive(Debug)]
pub enum Token {
    Int(isize),
    Bool(bool),
    Op(Operator),
    Sym(Sym),
    If,
    Then,
    Else
}

pub fn tokenize<'a>(chars: &'a [u8]) -> anyhow::Result<Vec<Token>> {
    match chars {
        [first, rest @ ..] if first.is_ascii_whitespace() => tokenize(rest),
        [b'1'..=b'9', _rest @ ..] => {
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
