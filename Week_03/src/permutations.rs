struct Solution;

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn back_track(nums: &mut Vec<i32>, start: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                ans.push(path.clone());
                return;
            }
            for i in start..nums.len() {
                path.push(nums[i]);
                nums.swap(start, i);
                back_track(nums, start+1, path, ans);
                nums.swap(start, i);
                path.pop();
            }
        }
        let mut ans = vec![];
        back_track(&mut nums, 0, &mut vec![], &mut ans);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        println!("{:?}", Solution::permute(vec![1, 2, 3]));
    }
}