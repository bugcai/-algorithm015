struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);
        while lo <= hi {
            let mid = (lo + hi) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[lo] <= nums[mid] {
                if nums[lo] <= target && target < nums[mid] {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[hi] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::search(vec![1], 2));
    }

    #[test]
    fn case4() {
        assert_eq!(1, Solution::search(vec![1, 3], 3));
    }

    #[test]
    fn case5() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8));
    }
}