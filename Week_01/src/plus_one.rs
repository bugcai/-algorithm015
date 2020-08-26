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
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn plus_one_with_carry() {
        assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
    }
}