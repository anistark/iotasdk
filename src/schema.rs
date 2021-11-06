// Schema Tool
use std::fs;

pub fn init(name: String) {
    println!("Initialising Schema Tool for new smart contract {}...", name);
    // Create schema.yaml file with initial content.
    let data = fs::read_to_string("./static/schema-sample.yaml").expect("Unable to read file").replace("ContractName", &name);
    fs::write("schema.yaml", data).expect("Unable to write file");
    println!("Your new schema is now ready! Build something awesome.")
}

pub fn rust() {
    println!("Generating Rust Smart Contract Code...");
}

pub fn go() {
    println!("Generating Go Smart Contract Code...");
}

pub fn ts() {
    println!("Generating TypeScript Smart Contract Code...");
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}