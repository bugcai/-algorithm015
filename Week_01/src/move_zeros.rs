struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                if i != j {
                    nums.swap(i, j);
                }
                j += 1;
            }
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