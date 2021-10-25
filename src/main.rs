use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config: Config = parse_config(&args);

    println!("arg 1 {}", config.a1);
    println!("arg 2 {}", config.a2);
}

struct Config {
    a1: String,
    a2: String,
}

fn parse_config(args: &[String]) -> Config {
    let a1: String = args[1].clone();
    let a2: String = args[2].clone();

    Config { a1, a2 }
}
