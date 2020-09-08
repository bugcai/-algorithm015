struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        fn back_track(nums: &mut [i32], path: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                result.push(path.clone());
                return;
            }
            let mut numbers_seen = HashSet::new();
            for i in start..nums.len() {
                if numbers_seen.contains(&nums[i]) {
                    continue;
                }
                numbers_seen.insert(nums[i]);
                nums.swap(i, start);
                path.push(nums[start]);
                back_track(nums, path, start + 1, result);
                nums.swap(i, start);
                path.pop();
            }
        }
        let mut result = vec![];
        back_track(&mut nums[..], &mut vec![], 0, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        println!("{:?}", Solution::permute_unique(vec![1, 2, 2]));
    }
}