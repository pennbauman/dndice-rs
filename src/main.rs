// DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use std::env;
use std::process;

mod dice;

static VERSION: &str = env!("CARGO_PKG_VERSION");

// Generate a print one set of statistics
fn stats(stats_type: &str) -> [i32; 6] {
    // Standard Array
    if (stats_type == "std") || (stats_type == "standard") {
        return [15, 14, 13, 12, 10, 8]
    // 1d20 for each stat
    } else if (stats_type == "d20") || (stats_type == "1d20") {
        let d20 = dice::parse("d20");
        //println!("Stats:");
        let mut score: [i32; 6] = [0; 6];
        for i in 0..6 {
            let result = dice::roll(&d20);
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
        let d6 = dice::parse("d6");
        //println!("Stats:");
        let mut score: [i32; 6] = [0; 6];
        for i in 0..6 {
            let mut min: i32 = 20;
            let mut result: i32 = 0;
            let mut result_r: i32;
            for _ in 0..4 {
                result_r = dice::roll(&d6);
                result += result_r;
                if result_r < min {
                    min = result_r;
                }
            }
            result -= min;
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
        eprintln!("unknown stats type");
        process::exit(1);
    }
}

// Main function
fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    // Parse args
    let mut dice_args: Vec<String> = vec![];
    for i in 1..args.len() {
        if (args[i] == "--help") || (args[i] == "-h") {
            //print_help();
            println!("help");
            process::exit(0);
        } else if args[i] == "--version" {
            println!("DnDice version {}", VERSION);
            process::exit(0);
        } else {
            if args[i].chars().nth(0).unwrap() == '-' {
                let result = &args[i].parse::<i32>();
                if result.is_err() {
                    eprintln!("invalid option '{}'", args[i]);
                    process::exit(1);
                }
            }
            dice_args.push(args[i].to_string());
        }
    }

    if dice_args.len() > 0 {
        if dice_args[0] == "stats" {
            if dice_args.len() == 2 {
                println!("Stats:");
                for s in &stats(&dice_args[1]) {
                    print!("{:2} ", s);
                }
                println!();
            } else {
                eprintln!("wrong args for stats");
                process::exit(1);
            }
        } else {
            let mut dice_text: String = "".to_string();
            for d in dice_args {
                dice_text += &d;
            }
            let dice_roll = dice::parse(&dice_text);
            dice::print_dice(&dice_roll);
            println!();
            println!("{}", dice::roll(&dice_roll));
        }
    } else {
        eprintln!("no dice or command provided")
    }
}

