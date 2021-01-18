// log - DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use std::fmt;


// Results from Dice
#[derive(Debug)]
pub struct DiceRoll {
    rolled: i32,
    log: Vec<RollLog>,
}
impl DiceRoll {
    // Constructors
    pub fn new(x: i32) -> DiceRoll {
        DiceRoll { rolled: x, log: vec![] }
    }
    pub fn new_roll(x: i32, roll: RollLog) -> DiceRoll {
        DiceRoll { rolled: x, log: vec![roll] }
    }

    // Accessors
    pub fn num(&self) -> i32 {
        self.rolled
    }
    pub fn full_log(&self) -> String {
        if self.log.len() == 1 {
            format!("{}", self.log[0]);
        }
        let mut result = String::from("");
            //format!("| d{}: ");
        if self.log.len() == 1 {
                result.push_str(&format!("| {}", self.log[0]));
        } else {
            for l in &self.log {
                result.push_str(&format!("| d{}: {}", l.size(), l));
            }
        }
        return result;
    }

    // Mutators
    pub fn join(&mut self, other: &DiceRoll) {
        for l in &other.log {
            self.log.push(l.clone());
        }
    }
    pub fn add(&mut self, other: &DiceRoll) {
        self.rolled += other.rolled;
        self.join(&other);
    }
    pub fn sub(&mut self, other: &DiceRoll) {
        self.rolled -= other.rolled;
        self.join(&other);
    }
    pub fn mult(&mut self, other: &DiceRoll) {
        self.rolled *= other.rolled;
        self.join(&other);
    }
    pub fn mult_neg(&mut self, other: &DiceRoll) {
        self.rolled *= -1;
        self.mult(&other);
    }
}
impl fmt::Display for DiceRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.rolled);
    }
}


// Log of rolls preformed
#[derive(Debug)]
pub struct RollLog {
    size: i32,
    rolls: Vec<i32>,
}
impl RollLog {
    pub fn new(s: i32) -> Self {
        Self { size: s, rolls: vec![] }
    }
    pub fn size(&self) -> i32 {
        self.size
    }
    pub fn log(&mut self, s: i32) {
        if (s < 0) || (s > self.size) {
            panic!("Invalid number logged");
        }
        self.rolls.push(s);
    }
}
impl fmt::Display for RollLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::from("");
        for r in &self.rolls {
            result.push_str(&format!("{} ", r));
        }
        return write!(f, "{}", result);
    }
}
impl Clone for RollLog {
    fn clone(&self) -> Self {
        let mut fin = Self::new(self.size);
        for l in &self.rolls {
            fin.rolls.push(*l);
        }
        return fin;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // DiceRoll
    #[test]
    fn test_dice_roll_new() {
        let dr = DiceRoll::new(13);
        assert!(dr.num() == 13);
    }
    #[test]
    fn test_dice_roll_new_roll() {
        let mut rl = RollLog::new(4);
        rl.log(2);
        let dr = DiceRoll::new_roll(13, rl);
        assert!(dr.num() == 13);
        assert!(dr.log[0].rolls[0] == 2);
    }
    #[test]
    fn test_dice_roll_full_log() {
        let mut rl4 = RollLog::new(4);
        rl4.log(2);
        let dr4 = DiceRoll::new_roll(2, rl4);
        let mut rl8 = RollLog::new(8);
        rl8.log(7);
        rl8.log(3);
        let mut dr8 = DiceRoll::new_roll(10, rl8);
        dr8.join(&dr4);
        assert!("| d8: 7 3 | d4: 2 " == &dr8.full_log());
    }
    #[test]
    fn test_dice_roll_join() {
        let mut rl4 = RollLog::new(3);
        rl4.log(2);
        rl4.log(1);
        rl4.log(3);
        let dr3 = DiceRoll::new_roll(6, rl4);
        let mut rl12 = RollLog::new(12);
        rl12.log(10);
        let mut dr12 = DiceRoll::new_roll(10, rl12);
        dr12.join(&dr3);
        assert!(dr12.log[0].rolls[0] == 10);
        assert!(dr12.log[1].rolls[0] == 2);
        assert!(dr12.log[1].rolls[1] == 1);
        assert!(dr12.log[1].rolls[2] == 3);
    }
    #[test]
    fn test_dice_roll_add() {
        let mut dr7 = DiceRoll::new(7);
        let dr1 = DiceRoll::new(1);
        dr7.add(&dr1);
        assert!(dr7.num() == 8);
    }
    #[test]
    fn test_dice_roll_sub() {
        let mut dr9 = DiceRoll::new(9);
        let dr4 = DiceRoll::new(4);
        dr9.sub(&dr4);
        assert!(dr9.num() == 5);
    }
    #[test]
    fn test_dice_roll_mult() {
        let mut dr8 = DiceRoll::new(8);
        let dr2 = DiceRoll::new(2);
        dr8.mult(&dr2);
        assert!(dr8.num() == 16);
    }
    #[test]
    fn test_dice_roll_mult_neg() {
        let mut dr8 = DiceRoll::new(12);
        let dr2 = DiceRoll::new(3);
        dr8.mult_neg(&dr2);
        assert!(dr8.num() == -36);
    }
    #[test]
    fn test_dice_roll_fmt() {
        let dr = DiceRoll::new(5);
        assert!("5" == format!("{}", dr));
    }

    // RollLog
    #[test]
    fn test_roll_log_new() {
        let rl = RollLog::new(7);
        assert!(rl.size == 7);
        assert!(rl.rolls.len() == 0)
    }
    #[test]
    fn test_roll_log_log() {
        let mut rl = RollLog::new(12);
        rl.log(4);
        rl.log(1);
        rl.log(12);
        assert!(rl.rolls.len() == 3);
        assert!(rl.rolls[0] == 4);
        assert!(rl.rolls[1] == 1);
        assert!(rl.rolls[2] == 12);
    }

    #[test]
    #[should_panic]
    fn test_roll_log_log_panic1() {
        let mut rl = RollLog::new(4);
        rl.log(5);
    }
    #[test]
    #[should_panic]
    fn test_roll_log_log_panic2() {
        let mut rl = RollLog::new(100);
        rl.log(-2);
    }
    #[test]
    fn test_roll_log_fmt() {
        let mut rl = RollLog::new(6);
        rl.log(2);
        rl.log(5);
        assert_eq!("2 5 ", &format!("{}", rl));
    }
    #[test]
    fn test_roll_log_clone() {
        let mut rl = RollLog::new(20);
        rl.log(3);
        rl.log(1);
        rl.log(20);
        rl.log(19);
        let new_rl = rl.clone();
        assert!(rl.size == new_rl.size);
        assert!(rl.rolls.len() == new_rl.rolls.len());
        for i in 0..3 {
            assert!(rl.rolls[i] == new_rl.rolls[i]);
        }
    }
}
