// dice - DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use std::process;

extern crate rand;
use rand::Rng;

pub type Roll = Vec<(bool, Vec<(i32, i32)>)>;

pub fn parse(text: &str) -> Roll {
    let mut sum: Roll = vec![];
    for add in text.split("+") {
        let mut first = true;
        for sub in add.split("-") {
            let mut mult: Vec<(i32, i32)> = vec![];
            for m in sub.split("*") {
                let mut i: u8 = 0;
                let mut first: i32 = 0;
                let mut second: i32 = 1;
                for d in m.split("d") {
                    if (d == "") && (i == 0) {
                        first = 1;
                    } else {
                        let result = d.parse();
                        if result.is_err() {
                            eprintln!("invalid str '{}'", d);
                            process::exit(1);
                        }
                        if i == 0 {
                            first = result.unwrap();
                        } else {
                            second = result.unwrap();
                        }
                    }
                    i += 1;
                }
                mult.push((first, second));
            }
            sum.push((first, mult));
            first = false;
        }
    }
    return sum;
}

pub fn roll(r: &Roll) -> i32 {
    let mut sum: i32 = 0;
    for term in r {
        let mut product: i32 = 1;
        for mult in &term.1 {
            let mut roll: i32 = 0;
            if mult.1 == 1 {
                roll = mult.0;
            } else {
                for _r in 0..mult.0 {
                    roll += rand::thread_rng().gen_range(1, mult.1 + 1);
                    //println!("{}d{} {}", mult.0, mult.1, roll);
                }
            }
            product *= roll;
            //println!("mult {}", roll);
        }
        if term.0 {
            sum += product;
            //println!("add {}", product);
        } else {
            sum -= product;
            //println!("sub {}", product);
        }
    }
    return sum;
}

pub fn print_dice(r: &Roll) {
    let mut term_later = false;
    for term in r {
        if term_later {
            if term.0 {
                print!(" + ")
            } else {
                print!(" - ")
            }
        }
        let mut mult_later = false;
        for mult in &term.1 {
            if mult_later {
                print!(" x ");
            }
            print!("{}d{}", mult.0, mult.1);
            mult_later = true;
        }
        term_later = true;
    }
}

