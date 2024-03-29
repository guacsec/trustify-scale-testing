use std::env;
use std::process;
use std::process::ExitCode;

mod config;
mod replicator;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let config = config::Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    config.validate();

    println!("Replication multiplier {}", config.replicator);
    println!("Source directory {}", config.src);
    println!("Destination directory {}", config.dst);

    replicator::run(config).unwrap_or_else(|err| {
        println!("Application error: {err}");
        process::exit(1)
    });

    0.into()
}
