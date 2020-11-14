// DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use std::env;
use std::process;

use colored::*;

mod dice;


// Print error well formated
#[macro_export]
macro_rules! err {
    ( $a:expr ) => {
        eprintln!("{} {}", "Error:".red(), $a);
        process::exit(1);
    };
    ( $a:expr, $b:expr ) => {
        eprintln!("{} {} '{}'", "Error:".red(), $a, $b);
        process::exit(1);
    };
}


// Generate a print one set of statistics
fn stats(stats_type: &str) -> [i32; 6] {
    // Standard Array
    if (stats_type == "std") || (stats_type == "standard") {
        return [15, 14, 13, 12, 10, 8];
    // 1d20 for each stat
    } else if (stats_type == "d20") || (stats_type == "1d20") {
        let d20 = dice::parse("d20").unwrap();
        let mut score: [i32; 6] = [0; 6];
        for i in 0..6 {
            let result = dice::roll(&d20).0;
            // Sort stats list as new scores are added
            let mut j = i;
            loop {
                if (j == 0) || (score[j-1] >= result) {
                    score[j] = result;
                    break;
                }
                score[j] = score[j-1];
                j -= 1;
            }
        }
        return score;
    // Best 3 of 4d6 for each stat
    } else if (stats_type == "4d6") || (stats_type == "3d6") {
        let d6 = dice::parse("d6").unwrap();
        let mut score: [i32; 6] = [0; 6];
        for i in 0..6 {
            // Roll 3d4 subtract min
            let mut min: i32 = 20;
            let mut result: i32 = 0;
            let mut result_r: i32;
            for _ in 0..4 {
                result_r = dice::roll(&d6).0;
                result += result_r;
                if result_r < min {
                    min = result_r;
                }
            }
            result -= min;
            // Sort stats list as new scores are added
            let mut j = i;
            loop {
                if (j == 0) || (score[j-1] >= result) {
                    score[j] = result;
                    break;
                }
                score[j] = score[j-1];
                j -= 1;
            }
        }
        return score;
    // Unknown generation type
    } else {
        err!("Unknown statistics generation method");
    }
}

// Print help menu
fn print_help() {
    println!("help");
    // --help, -h
    // --version
    // --number, -n
    // --quiet, -q

    // stats [std,standard|d20,1d20|3d6,4d6]
    // dice
}


// Main function
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut dice_args: Vec<String> = vec![];
    let mut num_rolls: u16 = 1;
    let mut loud: bool = true;
    // Parse args
    let mut i: usize = 1;
    while i < args.len() {
        // Print help menu
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
                let mut scores = stats(&dice_args[1]);
                if loud {
                    println!("Stats:");
                }
                for _ in 0..num_rolls {
                    for s in &scores {
                        print!("{:2} ", s);
                    }
                    println!();
                    scores = stats(&dice_args[1]);
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
            for d in dice_args {
                for s in d.split_whitespace() {
                    dice_text += &s;
                }
            }
            let dice_parse = dice::parse(&dice_text);
            if dice_parse.is_err() {
                err!(dice_parse.unwrap_err());
            }
            let dice_roll = dice_parse.unwrap();
            // Roll dice
            for _ in 0..num_rolls {
                let dice_result = dice::roll(&dice_roll);
                if loud {
                    print!("{} ", dice::print_dice(&dice_roll));
                    println!("{}", dice_result.1);
                    print!("Result: ");
                }
                println!("{}", dice_result.0);
            }
        }
    } else {
        err!("No dice or command provided");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_std() {
        let expected = [15, 14, 13, 12, 10, 8];
        assert_eq!(expected, stats("std"));
        assert_eq!(expected, stats("standard"));
    }

    #[test]
    fn test_stats_1d20() {
        let result = stats("1d20");
        for _ in 1..10 {
            for i in 0..6 {
                assert!(result[i] > 0);
                assert!(result[i] <= 20);
            }
        }
    }
    #[test]
    fn test_stats_d20() {
        let result = stats("d20");
        for _ in 1..10 {
            for i in 0..6 {
                assert!(result[i] > 0);
                assert!(result[i] <= 20);
            }
        }
    }

    #[test]
    fn test_stats_4d6() {
        let result = stats("4d6");
        for _ in 1..10 {
            for i in 0..6 {
                assert!(result[i] >= 3);
                assert!(result[i] <= 18);
            }
        }
    }
    #[test]
    fn test_stats_3d6() {
        let result = stats("3d6");
        for _ in 1..10 {
            for i in 0..6 {
                assert!(result[i] >= 3);
                assert!(result[i] <= 18);
            }
        }
    }
}
