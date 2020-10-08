struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut right_most = 0;
        for (i, &step) in nums.iter().enumerate() {
            if i > right_most {
                return false;
            }
            right_most = right_most.max(i + step as usize);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }
}