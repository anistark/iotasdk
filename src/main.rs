#[macro_use]
extern crate clap;

mod nodes;
mod info;
mod schema;

use clap::Parser;

#[derive(Parser)]

#[clap(name = crate_name!())]
#[clap(version = crate_version!())]
#[clap(author = crate_authors!())]
#[clap(about = crate_description!())]

struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "default.conf")]
    config: String,

    /// A level of verbosity, and can be used multiple times
    // #[clap(long, parse(from_occurrences))]
    // verbose: i32,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(version = "0.0.1")]
    #[clap(author = "Kumar Anirudha <mail@anirudha.dev>")]

    /// Setup and Configure IOTA Nodes. Try: `iotasdk node --help`
    Node(Node),

    /// Initialise IOTASDK. Try: `iotasdk init --help`
    Init(Init),

    /// Deploy and Interact with Smart Contracts with VM specific and Chain Specific configs. Try: `iotasdk contract --help`
    Contract(Contract),

    /// Get all info. Try: `iotasdk info --help`
    Info(Info),

    /// Schema Tool is used to generate Smart Contract Code. Try: `iotasdk schema --help`
    Schema(Schema),
}

/// Configure and Control your IOTA Nodes
#[derive(Parser)]
struct Node {
    /// Print debug info
    #[clap(short)]
    debug: bool,

    /// Install Node
    #[clap(short, long)]
    install: bool,

    /// Update Node
    #[clap(short, long)]
    update: bool,

    /// Upgrade Node
    #[clap(long)]
    upgrade: bool,

    /// Reset Node
    #[clap(short, long)]
    reset: bool,

    /// Purge Node
    #[clap(short, long)]
    purge: bool,

    /// Node Input. Available: bee, hornet, goshimmer, wasp
    nodeselect: String,
}

/// Initialise IOTASDK
#[derive(Parser)]
struct Init {
    /// Config File
    #[clap(short, long, default_value="default.conf")]
    config: String

    // Other config here
}

/// Manage Smart Contracts
#[derive(Parser)]
struct Contract {
    /// Print debug info
    #[clap(short)]
    debug: bool,
    
    /// Smart Contract VM
    #[clap(long)]
    vm: String,

    /// WASP Chain ID.
    #[clap(long)]
    chain: String,
}

/// Schema Tool
#[derive(Parser)]
struct Schema {
    /// Initilising Schema Tool
    #[clap(short, long)]
    init: bool,

    /// Smart Contract Name
    #[clap(short, long)]
    name: String,

    /// Generate Rust Smart Contract Code
    #[clap(long)]
    rust: bool,

    /// Generate Go Smart Contract Code
    #[clap(long)]
    go: bool,

    /// Generate TypeScript Smart Contract Code
    #[clap(long)]
    ts: bool,
}

/// Info
#[derive(Parser)]
struct Info {
    /// System Info
    #[clap(long)]
    sys: bool

    // Other info here
}

fn main() {
    let opts: Opts = Opts::parse();

    // println!("Using input file: {}", opts.input);

    // match opts.verbose {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     _ => println!("Don't be ridiculous"),
    // }

    match opts.subcmd {
        SubCommand::Node(t) => {
            if t.debug {
                println!("Printing debug info...");
            }

            let nodeselected = t.nodeselect;
            println!("Selected node {}", nodeselected); //TODO: Remove later.
            if nodeselected == "bee" {
                nodes::bee();
            } else if nodeselected == "hornet" {
                nodes::hornet();
            } else if nodeselected == "goshimmer" {
                nodes::goshimmer();
            } else if nodeselected == "wasp" {
                nodes::wasp();
            } else {
                println!("Unknown Node!");
            }

        }

        SubCommand::Init(_i) => {
            //TODO: Create config file if doesn't exist.
            //TODO: Print basic config.
            println!("Printing config info... {}", _i.config);
        }

        SubCommand::Contract(t) => {
            if t.debug {
                println!("Printing debug info...");
            }

            if t.vm == "rust" {
                println!("Rust Wasm VM");
            } else if t.vm == "go" {
                println!("TinyGo Wasm VM");
            } else if t.vm == "typescript" {
                println!("TypeScript AssemblyScript VM");
            } else if t.vm == "evm" {
                println!("EVM VM");
            } else {
                println!("Unknown VM found : {}", t.vm);
            }

        }

        SubCommand::Info(info) => {
            if info.sys {
                info::sys();
            }

        }

        SubCommand::Schema(s) => {
            if s.init {
                schema::init(s.name);
            } else if s.rust {
                schema::rust();
            } else if s.go {
                schema::go();
            } else if s.ts {
                schema::ts();
            } else {
                println!("Unknonwn command for Schema Tool. Try: `iotasdk schema --help`");
            }

        }

    }

}
