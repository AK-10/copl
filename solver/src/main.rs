use std::env;

use solver::tokenizer::tokenize;
use solver::parser::parse;
use solver::solver::solve;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let problem = args.get(1).expect("expect problem expression");
    //let answer = args.get(2).expect("expect answer expression");
    println!("* problem: {:?} *", problem);

    let prob_tokens = tokenize(problem.as_bytes())?;
    let prob_ast = parse(prob_tokens.as_slice())?;

    //let ans_tokens = tokenize(answer.as_bytes())?;
    //let ans_ast = parse(ans_tokens.as_slice())?;
    println!("{:#?}", prob_ast);
    solve(prob_ast);

    Ok(())
}
