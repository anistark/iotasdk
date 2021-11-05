#[macro_use]
extern crate clap;

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

    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,

    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,

    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    #[clap(version = "0.0.1", author = "Kumar Anirudha <mail@anirudha.dev>")]

    Node(Node),

    Init(Init),
}

/// Configure and Control your IOTA Nodes
#[derive(Parser)]
struct Node {
    /// Print debug info
    #[clap(short)]
    debug: bool
}

/// Initialise IOTASDK
#[derive(Parser)]
struct Init {
    /// Config File
    #[clap(short, long, default_value="default.conf")]
    config: String

    // Other config here
}

fn main() {
    let opts: Opts = Opts::parse();

    println!("Using input file: {}", opts.input);

    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be ridiculous"),
    }

    match opts.subcmd {
        SubCommand::Node(t) => {
            if t.debug {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }

        SubCommand::Init(_i) => {
            println!("Printing config info... {}", _i.config);
        }
    }

}
