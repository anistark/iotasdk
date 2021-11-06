// Schema Tool
use std::fs;

pub fn init(name: String) {
    println!("Initialising Schema Tool for new smart contract {}...", name);
    // Create schema.yaml file with initial content.
    //TODO: Generate Code Instead of copy pasting.
    let data = "name: ContractName
    description: My awesome ContractName
    structs: {}
    typedefs:
      AllowancesForAgent: map[AgentID]Int64
    state:
      allAllowances=a: map[AgentID]AllowancesForAgent
      balances=b: map[AgentID]Int64 // balances per account
      supply=s: Int64 // total supply of the token
    funcs:
      init:
        params:
          creator: AgentID // creator/owner of the initial supply
      transfer:
        params:
          account=ac: AgentID // target account
          amount=am: Int64 // amount of tokens to transfer
    views:
      balanceOf:
        params:
          account=ac: AgentID // sender account
        results:
          amount=am: Int64
      totalSupply:
        results:
          supply=s: Int64".replace("ContractName", &name);
    fs::write("schema.yaml", data).expect("Unable to write file");
    println!("Your new schema is now ready! Build something awesome.")
}

pub fn build(lang: String) {
    if lang == "rust" {
        println!("Generating Rust Smart Contract Code...");
    } else if lang == "go" {
        println!("Generating Go Smart Contract Code...");
    } else if lang == "ts" {
        println!("Generating TypeScript Smart Contract Code...");
    }
}
