use std::env;

use solver::tokenizer::tokenize;
use solver::parser::parse;
use solver::solver::solve;

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();


    if args.len() <= 1 {
        println!("usage:");
        println!("cargo run -- '<env>' '<expr>'\n");

        println!("example:");
        println!("cargo run -- 'x = 10, y = true' 'if x then y + 1 else y - 1'");

        return Ok(())
    }

    let env = args.get(1).expect("expect env");
    let expr = args.get(2).expect("expect expression");
    println!("* env: {:?} *", env);
    println!("* expr: {:?} *", expr);

    let env = tokenize(env.as_bytes())?;
    let expr = tokenize(expr.as_bytes())?;

    println!("{:?}", env);
    println!("{:?}", expr);

    let ast = parse(env.as_slice(), expr.as_slice())?;
    println!("{:#?}", ast);

    solve(&ast);

    Ok(())
}
