use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let config: Config = Config::new(&args).unwrap_or_else(op: |err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(code: 1);
    });

    println!("arg 1 {}", config.a1);
    println!("arg 2 {}", config.a2);
}

struct Config {
    a1: String,
    a2: String,
}

impl Config {

    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let a1: String = args[1].clone();
        let a2: String = args[2].clone();

        Ok(Config { a1, a2 })
    }

}
