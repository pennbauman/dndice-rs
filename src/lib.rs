// DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use std::str::FromStr;

mod dice;
use dice::DiceSet;
mod parse;
pub use parse::DiceParseError;
mod log;
use log::DiceRoll;
mod stats;
pub use stats::Stats;


#[derive(Debug)]
pub struct Dice {
    name: Option<String>,
    set: DiceSet,
    history: Vec<DiceRoll>,
}
impl Dice {
    pub fn new() -> Self {
        Self {
            name: None,
            set: DiceSet::new(),
            history: vec![],
        }
    }
    pub fn from<S: ToString>(text: S) -> Result<Self, <Dice as FromStr>::Err> {
        Self::from_str(&text.to_string())
    }
    pub fn name<S: ToString>(&mut self, new_name: S) {
        self.name = Some(new_name.to_string());
    }
    pub fn roll(&mut self) -> i32 {
        let result = self.set.roll();
        let r = result.num();
        self.history.push(result);
        return r;
    }
    pub fn log(&self, i: usize) -> String {
        self.history[self.history.len() - 1 - i].full_log()
    }
}
impl FromStr for Dice {
    type Err = DiceParseError;

    fn from_str(s: &str) -> Result<Self, <Dice as FromStr>::Err> {
        let dice_set = match DiceSet::parse(s) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };
        Ok(Self {
            name: None,
            set: dice_set,
            history: vec![],
        })
    }
}
impl std::fmt::Display for Dice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(s) => write!(f, "{}: {}", s, self.set),
            None => write!(f, "{}", self.set),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_new() {
        let d = Dice::new();
        assert!(d.name == None);
        assert!(match d.set{
            DiceSet::Const(n) => n == 0,
            _ => false,
        });
        assert!(d.history.len() == 0);
    }
    #[test]
    fn test_dice_name() {
        let mut d = Dice::new();
        d.name("tester");
        match d.name {
            Some(s) => assert!(s == "tester"),
            None => panic!(),
        }
    }
    #[test]
    fn test_dice_roll() {
        let mut d = Dice::from("3d4 - 1").unwrap();
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += d.roll();
        }
        assert!(sum <= 11*100);
        assert!(sum >= 2*100);
    }
    #[test]
    fn test_dice_log() {
        let mut d = Dice::from("1d6 + 3").unwrap();
        let mut output = [0; 5];
        let mut i = 4;
        loop {
            output[i] = d.roll();
            if i == 0 {
                break;
            }
            i -= 1;
        }
        for i in 0..5 {
            let log = d.log(i);
            assert!(log == format!("| {} ", output[i] - 3));
        }
    }

    // Blackbox
    #[test]
    fn test_dice_parsing_0() {
        let d = Dice::from("-1d4*2").unwrap();
        assert!(format!("{}", d) == "-1d4x2");
    }
    #[test]
    fn test_dice_parsing_1() {
        let d = Dice::from("8 *6d12 -4 + 3d6 ").unwrap();
        assert!(format!("{}", d) == "8x6d12 - 4 + 3d6");
    }
    #[test]
    fn test_dice_parsing_2() {
        let d = Dice::from("2 * 1d4x4d16").unwrap();
        assert!(format!("{}", d) == "2x1d4x4d16");
    }
    #[test]
    fn test_dice_parsing_3() {
        let d = Dice::from("1d8x 3 -4+3d6").unwrap();
        assert!(format!("{}", d) == "1d8x3 - 4 + 3d6");
    }
    #[test]
    fn test_dice_parsing_4() {
        let d = Dice::from("4d4-2d12+3d6-10d10+1d100-1d20+5d8").unwrap();
        assert!(format!("{}", d) == "4d4 - 2d12 + 3d6 - 10d10 + 1d100 - 1d20 + 5d8");
    }
    #[test]
    fn test_dice_parsing_5() {
        let d = Dice::from("10*5d4 + 1d7").unwrap();
        assert!(format!("{}", d) == "10x5d4 + 1d7");
    }
    #[test]
    fn test_dice_parsing_6() {
        let d = Dice::from("3").unwrap();
        assert!(format!("{}", d) == "3");
    }
    #[test]
    fn test_dice_parsing_7() {
        let d = Dice::from("d20 + 3").unwrap();
        println!("{}", d);
        assert!(format!("{}", d) == "1d20 + 3");
    }
    #[test]
    fn test_dice_parsing_8() {
        let d = Dice::from("0d3 - 4*0").unwrap();
        assert!(format!("{}", d) == "0d3 - 4x0");
    }
    #[test]
    fn test_dice_parsing_9() {
        let d = Dice::from("4*1D7*3 + 20").unwrap();
        assert!(format!("{}", d) == "4x1d7x3 + 20");
    }

    #[test]
    #[should_panic]
    fn test_dice_parsing_panic_0() {
        Dice::from("1**4").unwrap();
    }
    #[test]
    #[should_panic]
    fn test_dice_parsing_panic_1() {
        Dice::from("3+-1d6").unwrap();
    }
    #[test]
    #[should_panic]
    fn test_dice_parsing_panic_2() {
        Dice::from("0+-3").unwrap();
    }
    #[test]
    #[should_panic]
    fn test_dice_parsing_panic_3() {
        Dice::from("1dd4").unwrap();
    }
}
