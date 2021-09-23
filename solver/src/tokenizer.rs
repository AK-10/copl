use std::str;

// BNF
// EvalML1
// i ∈ int
// b ∈ {true, false}
// v ∈ Value ::= i | b
// e ∈ Exp ::= i | b | e op e | if e then e else e
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
pub enum Token {
    EvalTo,
    Int(usize),
    Bool(bool),
    Op(Operator),
    If,
    Then,
    Else
}

pub struct Tokenizer<'a> {
    input: &'a [u8],
    pos: usize
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Tokenizer {
            input,
            pos: 0
        }
    }

    pub fn tokenize(&mut self) -> anyhow::Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while self.pos < self.input.len() {
            if self.current_sub_string().starts_with("evalto".as_bytes()) {
                tokens.push(Token::EvalTo);
                self.next(6);
                continue
            }
            if self.current_sub_string().starts_with("if".as_bytes()) {
                tokens.push(Token::If);
                self.next(2);
                continue
            }
            if self.current_sub_string().starts_with("then".as_bytes()) {
                tokens.push(Token::Then);
                self.next(4);
                continue
            }
            if self.current_sub_string().starts_with("else".as_bytes()) {
                tokens.push(Token::Else);
                self.next(4);
                continue
            }
            if self.current_sub_string().starts_with("true".as_bytes()) {
                tokens.push(Token::Bool(true));
                self.next(4);
                continue
            }
            if self.current_sub_string().starts_with("false".as_bytes()) {
                tokens.push(Token::Bool(false));
                self.next(5);
                continue
            }
            if self.current().map(|x| x.is_ascii_whitespace()).unwrap_or(false) {
                self.next(1);
                continue
            }
            match self.input[self.pos] {
                b'1'..=b'9' => {
                    tokens.push(Token::Int(self.get_num()))
                }
                b'+' => {
                    tokens.push(Token::Op(Operator::Plus));
                    self.next(1);
                }
                b'-' => {
                    tokens.push(Token::Op(Operator::Minus));
                    self.next(1);
                }
                b'*' => {
                    tokens.push(Token::Op(Operator::Mul));
                    self.next(1);
                }
                b'<' => {
                    tokens.push(Token::Op(Operator::LessThan));
                    self.next(1);
                }
                x => {
                    println!("tokens: {:?}", tokens);
                    return Err(anyhow::anyhow!("unexpected char: {}, pos: {}", x as char, self.pos))
                }
            }

        }
        Ok(tokens)
    }

    fn next(&mut self, step: usize) {
        self.pos += step;
    }

    fn current(&self) -> Option<&u8> {
        self.input.get(self.pos)
    }

    fn get_num(&mut self) -> usize {
        let mut num = Vec::<u8>::new();
        while let Some(c) = self.current().filter(|x| x.is_ascii_digit()) {
            num.push(*c);
            self.next(1);
        }

        str::from_utf8(&num)
            .expect("invalid utf8 number").to_string().parse::<usize>().expect("")
    }

    fn current_sub_string(&self) -> &[u8] {
        &self.input[self.pos..self.input.len()]
    }
}
