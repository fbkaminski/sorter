use sorter::commands::{Commands, SortStrategy};
use std::time::Instant;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Fabio Kaminski", version, about = "simple integer sorter")]
struct Args {
    #[clap(subcommand)]
    cmd: Action
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    Generate{ file: String},
    Sort{strategy: String, input: String, output: String},
    Check{input: String, output: String}
}

fn main() {
    let args = Args::parse();
    let mut command = match args.cmd.clone() {
        Action::Check { input, output } => Commands::compare(input.as_str(), output.as_str()),
        Action::Sort { strategy, input, output } =>  {
            let strategy_algo = match strategy.as_str() {
                "rust" => {SortStrategy::NATIVE},
                "radix" => {SortStrategy::RADIX}
                "bit" => {SortStrategy::BITSORT}
                _ => {SortStrategy::BITSORT}
            };
            println!("sorting using {:?} algorithm", strategy_algo);
            Commands::sort(strategy_algo, input.as_str(), output.as_str())
        },
        Action::Generate { file } => {Commands::generate(file.as_str(), 1, 1_000_000 - 1)}
    };
    let start = Instant::now();
    command.run();
    println!(
        "time elapsed: {:?}",
        start.elapsed()
    );
}
