struct Solution;

/// 环状替换
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        if k == 0 {
            return;
        }
        let mut count = 0;
        let mut start = 0;
        while count < len {
            let (mut prev, mut prev_num) = (start, nums[start]);
            loop {
                let next = (prev + k) % len;
                let next_num = nums[next];
                nums[next] = prev_num;
                prev = next;
                prev_num = next_num;
                count += 1;
                if prev == start {
                    break;
                }
            }
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
        Self::reverse(nums, 0, len - 1);
        Self::reverse(nums, 0, k - 1);
        Self::reverse(nums, k, len - 1);
    }

    fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
        for i in 0..(end - start + 1) / 2 {
            nums.swap(start + i, end - i);
        }
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