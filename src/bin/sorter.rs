use std::env;
use std::time::Instant;
use sorter::commands::{Command, Commands, SortStrategy};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut command: Box<dyn Command>;
    if args.len() == 1 {
        println!("sorter: invalid number of arguments");
        return;
    }
    let start = Instant::now();
    match args[1].as_str() {
        "generate" => {
            if args.len() < 3 {
                println!("generate: invalid number of arguments");
                return;
            }
            let output_file = args[2].as_str();
            command = Commands::generate(output_file, 1, 1_000_000-1);
        }
        "sort" => {
            if args.len() < 4 {
                println!("sort: invalid number of arguments");
                return;
            }
            let input_file = args[2].as_str();
            let output_file = args[3].as_str();
            let mut strategy = SortStrategy::BITSORT;
            if args.len() >= 5 {
                let strategy_str = args[4].as_str();
                if strategy_str == "native" {
                    println!("using rust sort to sort the numbers");
                    strategy = SortStrategy::NATIVE;
                }
            }
            command = Commands::sort(strategy, input_file, output_file);
        }
        "check" => {
            if args.len() < 4 {
                println!("check: invalid number of arguments");
                return;
            }
            let input_file = args[2].as_str();
            let output_file = args[3].as_str();
            command = Commands::compare(input_file, output_file);
        }
        _ => {
            println!("command not found");
            return;
        }
    }
    command.run();
    println!("Time elapsed in {}: {:?}", args[1].as_str(), start.elapsed());
    return;
}