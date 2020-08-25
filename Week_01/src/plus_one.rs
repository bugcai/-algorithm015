struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut i = digits.len();
        while i > 0 {
            i -= 1;
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                return digits;
            }
        }
        digits.insert(0, 1);
        return digits;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular() {
        let initial = vec![1, 2, 3];
        let result = Solution::plus_one(initial);
        let expected = vec![1, 2, 4];

        let matching = expected.iter().zip(&result).filter(|&(a, b)| a == b).count();
        assert!(matching == expected.len() && matching == result.len());
    }

    #[test]
    fn plus_one_with_carry() {
        let initial = vec![9, 9, 9];
        let result = Solution::plus_one(initial);
        let expected = vec![1, 0, 0, 0];

        let matching = expected.iter().zip(&result).filter(|&(a, b)| a == b).count();
        assert!(matching == expected.len() && matching == result.len());
    }
}