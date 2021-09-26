use std::env;

use solver::tokenizer::tokenize;
use solver::parser::parse;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let problem = args.get(1).expect("expect problem expression");
    println!("* problem: {:?} *", problem);
    let tokens = tokenize(problem.as_bytes())?;
    println!("{:?}", tokens);
    let ast = parse(tokens.as_slice());
    println!("{:#?}", ast);

    Ok(())
}
