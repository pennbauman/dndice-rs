// DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use std::env;
use std::process;
use colored::*;
use dndice::{Dice, Stats};


// Print error well formatted
#[macro_export]
macro_rules! err {
    ( $a:expr ) => {
        eprintln!("{} {}", "Error:".red(), $a);
        eprintln!("  Use 'dndice --help' for more information");
        process::exit(1);
    };
    ( $a:expr, $b:expr ) => {
        eprintln!("{} {} '{}'", "Error:".red(), $a, $b);
        eprintln!("  Use 'dndice --help' for more information");
        process::exit(1);
    };
}


// Print help text
fn print_help() {
    println!("Usage: dndice [command] [dice] [options]");
    println!();
    println!("Commands:");
    println!("  dice [dice]         Roll provided dice, used if no command is provided");
    println!("  stats [method]      Generates a set of six statistics with the provided method");
    println!("    std, standard       Use the standard 5th edition statistics array");
    println!("    1d20                Roll 1d20 for each score");
    println!("    4d6                 Roll 4d6 and sum the largest 3 for each score");
    println!();
    println!("Dice Format:");
    println!("  Each expression uses dice sets, numbers, and the '+', '-', and '*' operators");
    println!("  Dice sets use '#d#' where the '#'s indicate dice quantity and size respectively.");
    println!("  Dice are rolled individually and their results summed and combined by operators");
    println!("  A '+' or '-' at the beginning indicates 1d20 will be added to the result");
    println!();
    println!("Options:");
    println!("  --help, -h          Print this help menu");
    println!("  --version           Print the version number");
    println!("  --number, -n [num]  Repeat command the provided number of times");
    println!("  --quiet, -q         Print only essential information from command");
    println!();
}


// Main
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut dice_args: Vec<String> = vec![];
    let mut num_rolls: u16 = 1;
    let mut loud: bool = true;
    // Parse args
    let mut i: usize = 1;
    while i < args.len() {
        // Print help text
        if (args[i] == "--help") || (args[i] == "-h") {
            print_help();
            process::exit(0);
        // Print version number
        } else if args[i] == "--version" {
            println!("DnDice version {}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        // Set number of rolls
        } else if (args[i] == "-n") || (args[i] == "--number") {
            let result = &args[i+1].parse::<u16>();
            if result.is_err() {
                err!("Invalid number", args[i+1]);
            }
            num_rolls = *result.as_ref().unwrap();
            i += 1
        // Stops most printing
        } else if (args[i] == "-q") || (args[i] == "--quiet") {
            loud = false;
        // Concatinate non option parameters
        } else {
            if (args[i] != "-") && (args[i].chars().nth(0).unwrap() == '-') {
                let result = &args[i].parse::<i32>();
                if result.is_err() {
                    err!("Invalid option", args[i]);
                }
            }
            dice_args.push(args[i].to_string());
        }
        i += 1;
    }

    if dice_args.len() > 0 {
        // Generate statistics
        if dice_args[0] == "stats" {
            if dice_args.len() == 2 {
                if loud {
                    println!("Stats:");
                }
                for _ in 0..num_rolls {
                    let scores = match Stats::new(&dice_args[1]) {
                        Ok(s) => s,
                        Err(_) => {
                            err!("Unknown statistics generation method");
                        },
                    };
                    println!("{}", scores);
                }
            } else if dice_args.len() > 2 {
                err!("Too many statistics generation methods provided");
            } else {
                err!("No statistics generation method provided");
            }
        // Roll dice
        } else {
            // Concatinate dice string
            let mut dice_text: String = "".to_string();
            let start_arg: usize;
            if dice_args[0] == "dice" {
                start_arg = 1;
            } else {
                start_arg = 0;
            }
            for i in start_arg..dice_args.len() {
                for s in dice_args[i].split_whitespace() {
                    dice_text += &s;
                }
            }
            let mut dice = match Dice::from(&dice_text) {
                Ok(d) => d,
                Err(e) => {
                    err!(e);
                },
            };
            // Roll dice
            for _ in 0..num_rolls {
                let dice_result = dice.roll();
                if loud {
                    print!("{} ", dice);
                    println!("{}", dice.log(0));
                    print!("Result: ");
                }
                println!("{}", dice_result);
            }
        }
    } else {
        err!("No dice or command provided");
    }
}
