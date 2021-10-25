use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let a1: &String = &args[1];
    let a2: &String = &args[2];
    println!("arg 1 {}", a1);
    println!("arg 2 {}", a2);
}
