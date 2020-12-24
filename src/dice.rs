// DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use rand::Rng;
use std::fmt;

// Generic Dice type
pub trait Dice: fmt::Display {
    fn parse(text: &str) -> Result<Self, String> where Self: Sized;
    fn roll(&self) -> DiceRoll;
}


// Log of rolls preformed
struct DiceRollLog {
    size: i32,
    rolls: Vec<i32>,
}
impl DiceRollLog {
    fn new(s: i32) -> DiceRollLog {
        DiceRollLog { size: s, rolls: vec![] }
    }
    fn copy(src: &DiceRollLog) -> DiceRollLog {
        let mut fin = DiceRollLog::new(src.size);
        for l in &src.rolls {
            fin.rolls.push(*l);
        }
        return fin;
    }
    fn log(&mut self, s: i32) {
        if (s < 0) || (s > self.size) {
            panic!("Invalid number logged");
        }
        self.rolls.push(s);
    }
}

// Results from Dice
pub struct DiceRoll {
    rolled: i32,
    log: Vec<DiceRollLog>,
}
impl DiceRoll {
    fn new(x: i32) -> DiceRoll {
        DiceRoll { rolled: x, log: vec![] }
    }
    fn join(&mut self, other: &DiceRoll) {
        for l in &other.log {
            self.log.push(DiceRollLog::copy(&*l));
        }
    }
    fn add(&mut self, other: &DiceRoll) {
        self.rolled += other.rolled;
        self.join(&other);
    }
    fn sub(&mut self, other: &DiceRoll) {
        self.rolled -= other.rolled;
        self.join(&other);
    }
    fn mult(&mut self, other: &DiceRoll) {
        self.rolled *= other.rolled;
        self.join(&other);
    }
}


// Single constant
pub struct ConstDie {
    value: i32,
}
impl ConstDie {
    fn new(x: i32) -> ConstDie {
        ConstDie { value: x }
    }
}
impl Dice for ConstDie {
    fn parse(text: &str) -> Result<ConstDie, String> {
        let result = text.parse();
        if result.is_err() {
            return Err(format!("Invalid number '{}'", text));
        }
        Ok(ConstDie::new(result.unwrap()))

    }
    fn roll(&self) -> DiceRoll {
        DiceRoll::new(self.value)
    }
}
impl fmt::Display for ConstDie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}


// Single set of same sized die
pub struct Die {
    number: i32,
    sides: i32
}
impl Die {
    fn new(n: i32, s: i32) -> Die {
        Die {
            number: n,
            sides: s,
        }
    }
}
impl Dice for Die {
    fn parse(text: &str) -> Result<Die, String> {
        let mut i: u8 = 0;
        let mut n: i32 = 0;
        let mut s: i32 = 1;
        for d in text.split("d") {
            if i == 2 {
                return Err(format!("Invalid die '{}'", text));
            }
            if (d == "") && (i == 0) {
                n = 1;
            } else {
                let result = d.parse();
                if result.is_err() {
                    return Err(format!("Invalid number '{}'", d));
                }
                if i == 0 {
                    n = result.unwrap();
                } else {
                    s = result.unwrap();
                }
            }
            i += 1;
        }
        Ok(Die::new(n, s))
    }
    fn roll(&self) -> DiceRoll {
        let mut result = DiceRoll::new(0);
        let mut rolls_log = DiceRollLog::new(self.sides);
        for _ in 0..self.number {
            let this_roll = rand::thread_rng().gen_range(1, self.sides + 1);
            result.rolled += this_roll;
            rolls_log.log(this_roll);
        }
        result.log.push(rolls_log);
        return result;
    }

}
impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}d{}", self.number, self.sides)
    }
}


// Multiplication type
enum Multiplier {
    Const(ConstDie),
    Die(Die),
}
pub struct MultDice {
    dice: Vec<Multiplier>,
}
impl MultDice {
    fn new() -> MultDice {
        MultDice { dice: vec![] }
    }
}
impl Dice for MultDice {
    fn parse(text: &str) -> Result<Self, String> where Self: Sized {
        let mut new = MultDice::new();
        for m in text.split(|c| c == 'x' || c == 'X' || c == '*') {
            let d = Die::parse(m);
            if d.is_ok() {
                new.dice.push(Multiplier::Die(d.unwrap()));
            } else {
                let c = ConstDie::parse(m)?;
                new.dice.push(Multiplier::Const(c));
            }
        }
        Ok(new)
    }
    fn roll(&self) -> DiceRoll {
        let mut result = DiceRoll::new(1);
        for m in &self.dice {
            match m {
                Multiplier::Const(c) => result.mult(&c.roll()),
                Multiplier::Die(d) => result.mult(&d.roll()),
            }
        }
        return result;
    }
}
impl fmt::Display for MultDice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}


// Summation type
enum Addend {
    Const(ConstDie),
    Plus(Die),
    Minus(Die),
    Multiplication(MultDice)
}
pub struct SumDice {
    dice: Vec<Addend>,
}
impl SumDice {
    fn new() -> SumDice {
        SumDice { dice: vec![] }
    }
}
impl Dice for SumDice {
    fn parse(_text: &str) -> Result<Self, String> where Self: Sized {
        todo!()
    }
    fn roll(&self) -> DiceRoll {
        todo!()
    }
}
impl fmt::Display for SumDice {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}


// Classic DnD dice type
pub struct ClassicDice {
    dice: SumDice
}
impl ClassicDice {
    fn new() -> ClassicDice {
        ClassicDice { dice: SumDice::new() }
    }
}
impl Dice for ClassicDice {
    fn parse(_text: &str) -> Result<Self, String> where Self: Sized {
        todo!()
    }
    fn roll(&self) -> DiceRoll {
        todo!()
    }
}
impl fmt::Display for ClassicDice {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // DiceRollLog
    #[test]
    fn test_dice_roll_log_new() {
        let log = DiceRollLog::new(5);
        assert_eq!(log.size, 5);
        assert_eq!(log.rolls.len(), 0);
    }
    #[test]
    fn test_dice_roll_log_log() {
        let mut log = DiceRollLog::new(12);
        log.log(7);
        assert_eq!(log.size, 12);
        assert_eq!(log.rolls[0], 7);
    }
    #[test]
    #[should_panic]
    fn test_dice_roll_log_log_fail() {
        let mut log = DiceRollLog::new(12);
        log.log(17);
    }
    #[test]
    fn test_dice_roll_log_copy() {
        let mut l1 = DiceRollLog::new(8);
        l1.log(2);
        l1.log(6);
        let l2 = DiceRollLog::copy(&l1);
        assert_eq!(l1.size, l2.size);
        assert_eq!(l1.rolls[0], l2.rolls[0]);
        assert_eq!(l1.rolls[1], l2.rolls[1]);
    }

    // DiceRoll
    #[test]
    fn test_dice_roll_new() {
        let r = DiceRoll::new(3);
        assert_eq!(r.rolled, 3);
        assert_eq!(r.log.len(), 0);
    }
    #[test]
    fn test_dice_roll_join() {
        let mut r1 = DiceRoll::new(0);
        r1.log.push(DiceRollLog::new(4));
        let mut r2 = DiceRoll::new(100);
        r2.log.push(DiceRollLog::new(10));
        r2.log.push(DiceRollLog::new(3));
        r1.join(&r2);
        assert_eq!(r1.rolled, 0);
        assert_eq!(r1.log.len(), 3);
    }
    #[test]
    fn test_dice_roll_add() {
        let mut r1 = DiceRoll::new(2);
        r1.log.push(DiceRollLog::new(7));
        let r2 = DiceRoll::new(40);
        r1.add(&r2);
        assert_eq!(r1.rolled, 42);
        assert_eq!(r1.log.len(), 1);
    }
    #[test]
    fn test_dice_roll_sub() {
        let mut r1 = DiceRoll::new(20);
        let mut r2 = DiceRoll::new(7);
        r2.log.push(DiceRollLog::new(9));
        r1.sub(&r2);
        assert_eq!(r1.rolled, 13);
        assert_eq!(r1.log.len(), 1);
    }
    #[test]
    fn test_dice_roll_mult() {
        let mut r1 = DiceRoll::new(5);
        r1.log.push(DiceRollLog::new(20));
        let mut r2 = DiceRoll::new(8);
        r2.log.push(DiceRollLog::new(1));
        r1.mult(&r2);
        assert_eq!(r1.rolled, 40);
        assert_eq!(r1.log.len(), 2);
    }

    // ConstDie
    #[test]
    fn test_const_die_new() {
        let c = ConstDie::new(7);
        assert_eq!(c.value, 7);
    }
    #[test]
    fn test_const_die_parse() {
        let c = ConstDie::parse("1").unwrap();
        assert_eq!(c.value, 1);
    }
    #[test]
    #[should_panic]
    fn test_const_die_parse_fail() {
        ConstDie::parse("&").unwrap();
    }
    #[test]
    fn test_const_die_roll() {
        let c = ConstDie::new(9);
        let r = c.roll();
        assert_eq!(r.rolled, 9);
    }

    // Die
    #[test]
    fn test_die_new() {
        let d = Die::new(1, 6);
        assert_eq!(d.number, 1);
        assert_eq!(d.sides, 6);
    }
    #[test]
    fn test_die_parse() {
        let d = Die::parse("3d4").unwrap();
        assert_eq!(d.number, 3);
        assert_eq!(d.sides, 4);
    }
    #[test]
    #[should_panic]
    fn test_die_parse_fail() {
        Die::parse("1d1d").unwrap();
    }
    #[test]
    fn test_die_roll() {
        let d = Die::parse("2d10").unwrap();
        let mut sum: i32 = 0;
        for _ in 1..100 {
            sum += d.roll().rolled;
        }
        assert!(sum <= 20*100);
        assert!(sum >= 2*100);
    }

    // MultDice
    #[test]
    fn test_mult_dice_new() {
        let m = MultDice::new();
        assert_eq!(m.dice.len(), 0);
    }

    // SumDice
    #[test]
    fn test_sum_dice_new() {
        let s = SumDice::new();
        assert_eq!(s.dice.len(), 0);
    }

    // ClassicDice
    #[test]
    fn test_classic_dice_new() {
        let c = ClassicDice::new();
        assert_eq!(c.dice.dice.len(), 0);
    }
}
