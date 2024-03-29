// stats - DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use crate::dice::Die;


/// A set of six ability scores
#[derive(Debug)]
pub struct Scores {
    nums: [u8; 6],
}
impl Scores {
    fn new(mut nums_array: [u8; 6]) -> Self {
        for s in &nums_array {
            if *s <= 0 {
                panic!("impossible stat")
            }
            if *s > 30 {
                panic!("impossible stat")
            }
        }
        nums_array.sort();
        nums_array.reverse();
        return Self { nums: nums_array };
    }
    /// Return scores generated with a method based on the string given
    ///
    /// # Methods
    /// * `std` or `standard`: use std()
    /// * `d20` or `1d20`: use d20()
    /// * `4d6` or `3d6`: use lowest3_4d6()
    pub fn from<S: ToString>(method: S) -> Result<Self, ()> {
        let m = method.to_string();
        if (m == "std") || (m == "standard") {
            Ok(Self::std())
        } else if (m == "d20") || (m == "1d20") {
            Ok(Self::d20())
        } else if (m == "4d6") || (m == "3d6") {
            Ok(Self::lowest3_4d6())
        } else {
            Err(())
        }
    }
    /// Return the 5th edition D&D standard ability scores
    pub fn std() -> Self {
        Self::new([15, 14, 13, 12, 10, 8])
    }
    /// Return scores generated by rolling 1d20 for each score
    pub fn d20() -> Self {
        let dice = Die::new(1, 20);
        let mut stats = [0; 6];
        for i in 0..6 {
            stats[i] = dice.roll().num().try_into().unwrap();
        }
        return Self::new(stats);
    }
    /// Return scores generated by rolling 4d6 and using the sum of the highest 3 number rolled for each score
    pub fn lowest3_4d6() -> Self {
        let dice = Die::new(1, 6);
        let mut stats = [0; 6];
        for i in 0..6 {
            let mut sum = 0;
            let mut min = dice.roll().num();
            for _ in 0..3 {
                let temp = dice.roll().num();
                if temp >= min {
                    sum += temp;
                } else {
                    sum += min;
                    min = temp;
                }
            }
            stats[i] = sum.try_into().unwrap();
        }
        return Self::new(stats);
    }
}
impl std::fmt::Display for Scores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:2} {:2} {:2} {:2} {:2} {:2}", self.nums[0], self.nums[1], self.nums[2],
               self.nums[3], self.nums[4], self.nums[5])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scores_std() {
        let expected = [15, 14, 13, 12, 10, 8];
        assert_eq!(expected, Scores::from("std").unwrap().nums);
        assert_eq!(expected, Scores::from("standard").unwrap().nums);
    }

    #[test]
    fn test_scores_1d20() {
        for _ in 1..10 {
            let result = Scores::from("1d20").unwrap().nums;
            for i in 0..6 {
                assert!(result[i] > 0);
                assert!(result[i] <= 20);
            }
        }
    }
    #[test]
    fn test_scores_d20() {
        for _ in 1..10 {
            let result = Scores::from("d20").unwrap().nums;
            for i in 0..6 {
                assert!(result[i] > 0);
                assert!(result[i] <= 20);
            }
        }
    }

    #[test]
    fn test_scores_4d6() {
        for _ in 1..10 {
            let result = Scores::from("4d6").unwrap().nums;
            for i in 0..6 {
                assert!(result[i] >= 3);
                assert!(result[i] <= 18);
            }
        }
    }
    #[test]
    fn test_scores_3d6() {
        for _ in 1..10 {
            let result = Scores::from("3d6").unwrap().nums;
            for i in 0..6 {
                assert!(result[i] >= 3);
                assert!(result[i] <= 18);
            }
        }
    }
}
