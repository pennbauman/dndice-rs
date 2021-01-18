// stats - DnDice
//   URL: https://github.com/pennbauman/dndice-rs
//   Author:
//     Penn Bauman (pennbauman@protonmail.com)
use crate::dice::Die;


#[derive(Debug)]
pub struct Stats {
    nums: [i32; 6],
}
impl Stats {
    pub fn new<S: ToString>(m: S) -> Result<Self, ()> {
        let method = m.to_string();
        if (method == "std") || (method == "standard") {
            Ok(Self::std())
        } else if (method == "d20") || (method == "1d20") {
            Ok(Self::d20())
        } else if (method == "4d6") || (method == "3d6") {
            Ok(Self::lowest3_4d6())
        } else {
            Err(())
        }
    }
    fn from(nums_array: [i32; 6]) -> Self {
        for s in &nums_array {
            if *s <= 0 {
                panic!("impossible stat")
            }
            if *s > 30 {
                panic!("impossible stat")
            }
        }
        return Self { nums: nums_array };
    }
    pub fn std() -> Self {
        Self::from([15, 14, 13, 12, 10, 8])
    }
    pub fn d20() -> Self {
        let dice = Die::new(1, 20);
        let mut stats = [0; 6];
        for i in 0..6 {
            stats[i] = dice.roll().num();
        }
        return Self::from(stats);
    }
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
            stats[i] = sum;
        }
        return Self::from(stats);
    }
}
impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:2} {:2} {:2} {:2} {:2} {:2}", self.nums[0], self.nums[1], self.nums[2],
               self.nums[3], self.nums[4], self.nums[5])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_std() {
        let expected = [15, 14, 13, 12, 10, 8];
        assert_eq!(expected, Stats::new("std").unwrap().nums);
        assert_eq!(expected, Stats::new("standard").unwrap().nums);
    }

    #[test]
    fn test_stats_1d20() {
        for _ in 1..10 {
            let result = Stats::new("1d20").unwrap().nums;
            for i in 0..6 {
                assert!(result[i] > 0);
                assert!(result[i] <= 20);
            }
        }
    }
    #[test]
    fn test_stats_d20() {
        for _ in 1..10 {
            let result = Stats::new("d20").unwrap().nums;
            for i in 0..6 {
                assert!(result[i] > 0);
                assert!(result[i] <= 20);
            }
        }
    }

    #[test]
    fn test_stats_4d6() {
        for _ in 1..10 {
            let result = Stats::new("4d6").unwrap().nums;
            for i in 0..6 {
                assert!(result[i] >= 3);
                assert!(result[i] <= 18);
            }
        }
    }
    #[test]
    fn test_stats_3d6() {
        for _ in 1..10 {
            let result = Stats::new("3d6").unwrap().nums;
            for i in 0..6 {
                assert!(result[i] >= 3);
                assert!(result[i] <= 18);
            }
        }
    }
}
