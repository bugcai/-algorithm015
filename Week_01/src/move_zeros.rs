struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_non_zero_found_at = 0;
        let mut cur = 0;
        while cur < nums.len() {
            if nums[cur] != 0 {
                if last_non_zero_found_at != cur {
                    nums.swap(last_non_zero_found_at, cur);
                }
                last_non_zero_found_at += 1;
            }
            cur += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular() {
        let mut array_with_zero = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut array_with_zero);
        let expected = vec![1, 3, 12, 0, 0];

        let matching = expected.iter().zip(&array_with_zero).filter(|&(a, b)| a == b).count();
        assert!(matching == expected.len() && matching == array_with_zero.len());
    }
}