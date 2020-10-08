struct Solution;

/// 环状替换
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize;
        let (size, mut count, mut start) = (nums.len(), 0, 0);
        while count < size {
            let mut target = (start + k) % size;
            while target != start {
                nums.swap(start, target);
                count += 1;
                target = (target + k) % size;
            }
            count += 1;
            start += 1;
        }
    }
}

struct Solution2;

/// 反转法
/// 1. 反转所有数字
/// 2. 反转前 k 个数字
/// 3. 反转后 n-k 个数字
impl Solution2 {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn case2() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution2::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }
}