#[macro_use]
extern crate clap;

mod nodes;
mod info;

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

    Node(Node),

    Init(Init),

    Contract(Contract),

    Info(Info),
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
    
    /// Smart Contract Project
    project: String,
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

        }

        SubCommand::Info(info) => {
            if info.sys {
                info::sys();
            }

        }
    }

}
