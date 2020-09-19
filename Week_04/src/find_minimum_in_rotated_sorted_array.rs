struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while nums[lo] > nums[hi] {
            let mid = (lo + hi) / 2;
            if nums[mid] >= nums[lo] { lo = mid + 1; } else { hi = mid }
        }
        nums[lo]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::find_min(vec![3, 4, 5, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::find_min(vec![2, 1]));
    }
}