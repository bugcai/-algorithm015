use std::collections::HashMap;

struct Solution;

// 一遍哈希表
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&pos) = map.get(&(target - num)) {
                return vec![pos as i32, i as i32];
            }
            map.insert(num, i);
        }
        panic!()
    }
}

struct Solution2;

// 双指针法
impl Solution2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut tmp = nums.clone();
        tmp.sort();
        let mut res = vec![];
        let (mut lo, mut hi) = (0, tmp.len() - 1);
        while lo < hi {
            let sum = tmp[lo] + tmp[hi];
            if sum > target {
                hi -= 1;
            } else if sum < target {
                lo += 1;
            } else {
                for k in 0..nums.len() {
                    if nums[k] == tmp[lo] || nums[k] == tmp[hi] {
                        res.push(k as i32);
                    }
                }
                return res;
            }
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution2::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}