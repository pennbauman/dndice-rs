// dice - DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use rand::Rng;


// Rollable dice type
pub type Roll = Vec<(bool, Vec<(i32, i32)>)>;

// Parse string into dice roll
pub fn parse(text: &str) -> Result<Roll, String> {
    let mut sum: Roll = vec![];
    let mut start = true;
    for add in text.split("+") {
        let mut first = true;
        for sub in add.split("-") {
            let mut mult: Vec<(i32, i32)> = vec![];
            if sub == "" {
                if start {
                    mult.push((1, 20));
                } else {
                    return Err("Invalid math".to_string());
                }
            } else {
                for m in sub.split("*") {
                    if m == "" {
                        return Err("Invalid math".to_string());
                    }
                    let mut i: u8 = 0;
                    let mut first: i32 = 0;
                    let mut second: i32 = 1;
                    for d in m.split("d") {
                        if (d == "") && (i == 0) {
                            first = 1;
                        } else {
                            let result = d.parse();
                            if result.is_err() {
                                return Err(format!("Invalid number '{}'", d));
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
            }
            sum.push((first, mult));
            first = false;
            start = false;
        }
    }
    return Ok(sum);
}

// Roll provided dice
pub fn roll(r: &Roll) -> (i32, String) {
    let mut sum: i32 = 0;
    let mut all_rolls: Vec<(i32, Vec<i32>)> = vec![];
    for term in r {
        let mut product: i32 = 1;
        for mult in &term.1 {
            let mut roll: i32 = 0;
            if mult.1 == 1 {
                roll = mult.0;
            } else {
                let mut current_rolls: Vec<i32> = vec![];
                for _r in 0..mult.0 {
                    let this_roll = rand::thread_rng().gen_range(1, mult.1 + 1);
                    //println!("{}d{} {}", mult.0, mult.1, roll);
                    roll += this_roll;
                    current_rolls.push(this_roll);
                }
                all_rolls.push((mult.1, current_rolls));
            }
            product *= roll;
        }
        if term.0 {
            sum += product;
        } else {
            sum -= product;
        }
    }
    let mut rolls_text: String = "".to_string();
    if all_rolls.len() > 1 {
        for d in all_rolls {
            rolls_text += &format!("| d{}: ", d.0);
            for r in d.1 {
                rolls_text += &format!("{} ", r);
            }
        }
    } else {
        rolls_text += "| ";
        for r in &all_rolls[0].1 {
            rolls_text += &format!("{} ", r);
        }
    }
    //println!("{}", rolls_text);
    return (sum, rolls_text);
}

pub fn print_dice(r: &Roll) -> String {
    let mut text: String = "".to_string();
    let mut term_later = false;
    for term in r {
        if term_later {
            if term.0 {
               text += " + ";
            } else {
                text += " - ";
            }
        }
        let mut mult_later = false;
        for mult in &term.1 {
            if mult_later {
                text += " x ";
            }
            if mult.1 == 1 {
                text += &format!("{}", mult.0);
            } else {
                text += &format!("{}d{}", mult.0, mult.1);
            }
            mult_later = true;
        }
        term_later = true;
    }
    return text;
}


#[cfg(test)]
mod tests {
    use super::*;

    // parse()
    #[test]
    fn test_parse_1d6() {
        let expected: Roll = vec![
            (true, vec![(1, 6)])
        ];
        assert_eq!(expected, parse("1d6").unwrap());
    }
    #[test]
    fn test_parse_d8() {
        let expected: Roll = vec![
            (true, vec![(1, 8)])
        ];
        assert_eq!(expected, parse("d8").unwrap());
    }
    #[test]
    fn test_parse_1d4_plus_2() {
        let expected: Roll = vec![
            (true, vec![(1, 4)]),
            (true, vec![(2, 1)])
        ];
        assert_eq!(expected, parse("1d4+2").unwrap());
    }
    #[test]
    fn test_parse_1d12_minus_7() {
        let expected: Roll = vec![
            (true, vec![(1, 12)]),
            (false, vec![(7, 1)])
        ];
        assert_eq!(expected, parse("1d12-7").unwrap());
    }
    #[test]
    fn test_parse_2d4_times_10() {
        let expected: Roll = vec![
            (true, vec![(2, 4), (10, 1)])
        ];
        assert_eq!(expected, parse("2d4*10").unwrap());
    }
    #[test]
    fn test_parse_plus_5() {
        let expected: Roll = vec![
            (true, vec![(1, 20)]),
            (true, vec![(5, 1)])
        ];
        assert_eq!(expected, parse("+5").unwrap());
    }
    #[test]
    fn test_parse_all() {
        let expected: Roll = vec![
            (true, vec![(1, 20)]),
            (true, vec![(1, 7)]),
            (false, vec![(5, 1), (8, 12)]),
            (true, vec![(3, 1)])
        ];
        assert_eq!(expected, parse("+d7-5*8d12+3").unwrap());
    }
    #[test]
    fn test_parse_invalid_math() {
        let result = parse("1d4+");
        assert!(result.is_err());
        assert_eq!("Invalid math".to_string(), result.unwrap_err());
    }
    #[test]
    fn test_parse_invalid_number() {
        let result = parse("1da");
        assert!(result.is_err());
        assert_eq!("Invalid number 'a'".to_string(), result.unwrap_err());
    }

    // roll()
    #[test]
    fn test_roll_1d20() {
        let d: Roll = vec![
            (true, vec![(1, 20)])
        ];
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += roll(&d).0;
        }
        assert!(sum <= 2000);
        assert!(sum >= 100);
    }
    #[test]
    fn test_roll_1d6_plus_3() {
        let d: Roll = vec![
            (true, vec![(1, 6)]),
            (true, vec![(3, 1)])
        ];
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += roll(&d).0;
        }
        assert!(sum <= 900);
        assert!(sum >= 400);
    }
    #[test]
    fn test_roll_1d4_minus_2() {
        let d: Roll = vec![
            (true, vec![(1, 4)]),
            (false, vec![(2, 1)])
        ];
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += roll(&d).0;
        }
        assert!(sum <= 200);
        assert!(sum >= -100);
    }
    #[test]
    fn test_roll_1d7_times_3() {
        let d: Roll = vec![
            (true, vec![(1, 7), (3, 1)])
        ];
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += roll(&d).0;
        }
        assert!(sum <= 2100);
        assert!(sum >= 200);
    }
    #[test]
    fn test_roll_all() {
        let d: Roll = vec![
            (true, vec![(7, 20)]),
            (false, vec![(1, 7)]),
            (false, vec![(5, 1), (8, 12)]),
            (true, vec![(3, 1), (5, 2), (8, 1)])
        ];
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += roll(&d).0;
        }
        assert!(sum <= 31900);
        assert!(sum >= -36000);
    }

    // print_dice()
    #[test]
    fn test_print_1d8() {
        let d: Roll = vec![
            (true, vec![(1, 8)])
        ];
        assert_eq!("1d8", print_dice(&d));
    }
    #[test]
    fn test_print_2d6_plus_5() {
        let d: Roll = vec![
            (true, vec![(2, 6)]),
            (true, vec![(5, 1)])
        ];
        assert_eq!("2d6 + 5", print_dice(&d));
    }
    #[test]
    fn test_print_3d12_minus_1d8_plus_2() {
        let d: Roll = vec![
            (true, vec![(3, 12)]),
            (false, vec![(1, 8)]),
            (true, vec![(2, 1)])
        ];
        assert_eq!("3d12 - 1d8 + 2", print_dice(&d));
    }
    #[test]
    fn test_print_10_times_5d6() {
        let d: Roll = vec![
            (true, vec![(10, 1), (5, 6)]),
        ];
        assert_eq!("10 x 5d6", print_dice(&d));
    }
}
