// dice - DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use std::fmt;
use rand::Rng;
use crate::log::{DiceRoll, RollLog};
use crate::parse::{ParseKind, ParseState, DiceParseError};


// Dice Expression
#[derive(Debug)]
pub enum DiceSet {
    Sum(DiceSeries),
    Mult(DiceSeries),
    Die(Die),
    Const(i32),
}
impl DiceSet {
    pub fn new() -> Self {
        Self::Const(0)
    }
    pub fn parse(text: &str) -> Result<Self, DiceParseError> {
        let parser = match ParseState::parse_from(text) {
            Ok(p) => p,
            Err(e) => return Err(DiceParseError::InvalidChar(e)),
        };
        let splits = parser.splits();
        match parser.kind() {
            ParseKind::Sum => {
                let mut series: DiceSeries = vec![];
                for s in splits {
                    if s.0 == '+' {
                        match SignedDice::parse_pos(&s.1) {
                            Ok(d) => series.push(d),
                            Err(e) => return Err(e),
                        };
                    } else if s.0 == '-' {
                        match SignedDice::parse_neg(&s.1) {
                            Ok(d) => series.push(d),
                            Err(e) => return Err(e),
                        };
                    } else {
                        panic!("invalid sum");
                    }
                }
                return Ok(DiceSet::Sum(series));
            },
            ParseKind::Mult => {
                let mut series: DiceSeries = vec![];
                for s in splits {
                    if s.1 == "" {
                        return Err(DiceParseError::InvalidMath(String::from(text)));
                    }
                    let start = s.1.chars().next().unwrap();
                    if start == '-' {
                        match SignedDice::parse_neg(&s.1[1..]) {
                            Ok(d) => series.push(d),
                            Err(e) => return Err(e),
                        }
                    } else {
                        match SignedDice::parse_pos(&s.1) {
                            Ok(d) => series.push(d),
                            Err(e) => return Err(e),
                        }
                    }
                }
                return Ok(DiceSet::Mult(series));
            },
            ParseKind::Die => {
                if splits.len() == 2 {
                    let mut num = 1;
                    if splits[0].1 != "" {
                        num = match splits[0].1.parse::<i32>() {
                            Ok(i) => i,
                            Err(_) => return Err(DiceParseError::InvalidNumber(
                                splits[0].1.to_string()
                            )),
                        };
                    }
                    let size = match splits[1].1.parse::<i32>() {
                        Ok(i) => i,
                        Err(_) => return Err(DiceParseError::InvalidNumber(
                            splits[1].1.to_string()
                        )),
                    };
                    return Ok(Self::Die(Die::new(num, size)));
                } else {
                    println!("'{}' {:?}", text, splits);
                    return Err(DiceParseError::InvalidDie(String::from(text)));
                }
            },
            ParseKind::Const => {
                match text.parse::<i32>() {
                    Ok(i) => return Ok(Self::Const(i)),
                    Err(_) => return Err(DiceParseError::InvalidNumber(
                            String::from(text)
                    )),
                }
            },
        }
    }

    pub fn roll(&self) -> DiceRoll {
        match self {
            Self::Mult(series) => {
                let mut result = DiceRoll::new(1);
                for d in series {
                    match d {
                        SignedDice::Pos(x) => result.mult(&x.roll()),
                        SignedDice::Neg(x) => result.mult_neg(&x.roll()),
                    }
                }
                return result;
            },
            Self::Sum(series) => {
                let mut result = DiceRoll::new(0);
                for d in series {
                    match d {
                        SignedDice::Pos(x) => result.add(&x.roll()),
                        SignedDice::Neg(x) => result.sub(&x.roll()),
                    }
                }
                return result;
            },
            Self::Die(d) => d.roll(),
            Self::Const(x) => DiceRoll::new(*x),
        }
    }
}
impl fmt::Display for DiceSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("");
        let mut i: u8 = 0;
        match self {
            Self::Sum(s) => {
                for d in s {
                    if i == 0 {
                        match d {
                            SignedDice::Pos(d) => result.push_str(&format!("{}", d)),
                            SignedDice::Neg(d) => result.push_str(&format!("-{}", d)),
                        }
                    } else {
                        match d {
                            SignedDice::Pos(d) => result.push_str(&format!(" + {}", d)),
                            SignedDice::Neg(d) => result.push_str(&format!(" - {}", d)),
                        }
                    }
                    i += 1;
                }
                return write!(f, "{}", result);
            },
            Self::Mult(s) => {
                for d in s {
                    if i == 0 {
                        match d {
                            SignedDice::Pos(d) => result.push_str(&format!("{}", d)),
                            SignedDice::Neg(d) => result.push_str(&format!("-{}", d)),
                        }
                    } else {
                        match d {
                            SignedDice::Pos(d) => result.push_str(&format!("x{}", d)),
                            SignedDice::Neg(d) => result.push_str(&format!("x-{}", d)),
                        }
                    }
                    i += 1;
                }
                return write!(f, "{}", result);
            },
            Self::Die(d) => return write!(f, "{}", d),
            Self::Const(n) => return write!(f, "{}", n),
        }
    }
}


// Dice Array
type DiceSeries = Vec<SignedDice>;


// Signed Dice Set
#[derive(Debug)]
pub enum SignedDice {
    Pos(DiceSet),
    Neg(DiceSet),
}
impl SignedDice {
    fn parse_pos(text: &str) -> Result<SignedDice, DiceParseError> {
        Ok(SignedDice::Pos(DiceSet::parse(text)?))
    }
    fn parse_neg(text: &str) -> Result<SignedDice, DiceParseError> {
        Ok(SignedDice::Neg(DiceSet::parse(text)?))
    }
}


// Dice with one size
#[derive(Debug)]
pub struct Die {
    number: i32,
    sides: i32
}
impl Die {
    pub fn new(n: i32, s: i32) -> Die {
        Die {
            number: n,
            sides: s,
        }
    }
    pub fn roll(&self) -> DiceRoll {
        let mut sum = 0;
        let mut log = RollLog::new(self.sides);
        for _ in 0..self.number {
            let r = rand::thread_rng().gen_range(1, self.sides + 1);
            sum += r;
            log.log(r);
        }
        let result = DiceRoll::new_roll(sum, log);
        return result;
    }

}
impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.number, self.sides)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // DiceSet (whitebox)
    #[test]
    fn test_dice_set_new() {
        let x = match DiceSet::new() {
            DiceSet::Const(n) => n,
            _ => panic!(),
        };
        assert!(x == 0);
    }
    #[test]
    fn test_dice_set_parse() {
        let ds = DiceSet::parse("-1d8+2*-4-3").unwrap();
        match ds {
            DiceSet::Sum(s) => {
                // -1d8
                match &s[0] {
                    SignedDice::Neg(x) => match x {
                        DiceSet::Die(d) => {
                            if format!("{}", d) != "1d8" {
                                panic!();
                            }
                        },
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
                // 2*-4
                match &s[1] {
                    SignedDice::Pos(x) => match x {
                        DiceSet::Mult(arr) => {
                            match &arr[0] {
                                SignedDice::Pos(n) => match n {
                                    DiceSet::Const(i) => assert!(*i == 2),
                                    _ => panic!(),
                                },
                                _ => panic!(),
                            }
                            match &arr[1] {
                                SignedDice::Neg(n) => match n {
                                    DiceSet::Const(i) => assert!(*i == 4),
                                    _ => panic!(),
                                },
                                _ => panic!(),
                            }
                        },
                        _ => panic!(),
                    }
                    _ => panic!(),
                }
                // -3
                match &s[2] {
                    SignedDice::Neg(x) => match x {
                        DiceSet::Const(i) => assert!(*i == 3),
                        _ => panic!(),
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }
    #[test]
    fn test_dice_set_parse_err() {
        let ds = DiceSet::parse("1f4+ 2");
        assert!(match ds {
            Ok(_) => false,
            Err(e) => match e {
                DiceParseError::InvalidChar(c) => c == 'f',
                _ => false,
            },
        });
    }
    #[test]
    fn test_dice_set_roll() {
        let ds = DiceSet::parse("5d6 - 1d8 + 7").unwrap();
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += ds.roll().num();
        }
        assert!(sum <= 36*100);
        assert!(sum >= 4*100);
    }
    #[test]
    fn test_dice_set_fmt() {
        let ds = DiceSet::parse("5*3d4 + 1d12").unwrap();
        assert!(format!("{}", ds) == "5x3d4 + 1d12")
    }

    // SignedDice
    #[test]
    fn test_signed_dice_pos() {
        let sd = SignedDice::parse_pos("4").unwrap();
        assert!(match sd {
            SignedDice::Pos(p) => match p {
                DiceSet::Const(x) => x == 4,
                _ => false,
            },
            SignedDice::Neg(_) => false,
        });
    }
    #[test]
    fn test_signed_dice_neg() {
        let sd = SignedDice::parse_neg("7d8").unwrap();
        println!("{:?}", sd);
        assert!(match sd {
            SignedDice::Pos(_) => false,
            SignedDice::Neg(p) => match p {
                DiceSet::Die(d) => "7d8" == &format!("{}", d),
                _ => false,
            },
        });
    }

    // Die
    #[test]
    fn test_die_new() {
        let d = Die::new(1, 6);
        assert_eq!(d.number, 1);
        assert_eq!(d.sides, 6);
    }
    #[test]
    fn test_die_roll() {
        let d = Die::new(2, 10);
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += d.roll().num();
        }
        assert!(sum <= 20*100);
        assert!(sum >= 2*100);
    }
    #[test]
    fn test_die_fmt() {
        let d = Die::new(3, 4);
        assert!("3d4" == format!("{}", d));
    }
}
