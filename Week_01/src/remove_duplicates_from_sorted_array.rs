struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        let mut uniqIdx = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[uniqIdx] {
                if i - uniqIdx > 1 { nums.swap(i, uniqIdx + 1); }
                uniqIdx += 1;
            }
        }
        (uniqIdx + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::remove_duplicates(&mut vec![1, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]));
    }
}