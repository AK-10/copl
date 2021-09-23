use std::env;

use solver::tokenizer::Tokenizer;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let problem = args.get(1).expect("expect problem expression");
    println!("* problem: {:?} *", problem);
    let mut tokenizer = Tokenizer::new(problem.as_bytes());
    let tokens = tokenizer.tokenize()?;

    println!("{:?}", tokens);

    Ok(())
}
