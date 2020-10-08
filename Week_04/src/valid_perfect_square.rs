struct Solution;

/// 完全平方数可以通过累加从1往后的奇数找到，
///
/// 1 = 1;
/// 4 = 1 + 3;
/// 9 = 1 + 3 + 5;
/// 16 = 1 + 3 + 5 + 7;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut num = num;
        let mut i = 1;
        while (num > 0) {
            num -= i;
            i += 2;
        }
        num == 0
    }
}


struct Solution2;

/// 牛顿迭代法
impl Solution2 {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut guess: i64 = num as i64 / 2;
        while guess * guess > num as i64 {
            guess = (guess + num as i64 / guess) / 2;
        }
        num == 1 || guess * guess == num as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_perfect_square(16));
        assert!(Solution2::is_perfect_square(16));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_perfect_square(14));
        assert!(!Solution2::is_perfect_square(14));
    }
}