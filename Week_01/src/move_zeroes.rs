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
    fn case() {
        let mut array_with_zero = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut array_with_zero);
        assert_eq!(array_with_zero, vec![1, 3, 12, 0, 0]);
    }
}