use clap::Parser;

#[derive(Parser, Debug)]

#[clap(name = "IOTASDK")]
#[clap(version = "0.0.1")]
#[clap(author = "Kumar Anirudha github.com/anistark")]
#[clap(about = "Simplifying BUIDL using IOTA!")]

struct Opts {
    /// Command to run
    #[clap(short, long)]
    command: String,

    /// Number of times to greet
    #[clap(long, default_value = "1")]
    count: u8,
}

fn main() {
    let options = Opts::parse();

    for n in 0..options.count {
        println!("Command is {} for count {}!", options.command, n+1)
    }
}
