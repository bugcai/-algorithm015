struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn back_track(start: i32, end: i32, k: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if k == 0 {
                ans.push(path.clone());
                return;
            }
            for i in start..=end - k + 1 {
                path.push(i);
                back_track(i + 1, end, k - 1, path, ans);
                path.pop();
            }
        }
        let mut ans = vec![];
        back_track(1, n, k, &mut vec![], &mut ans);
        ans
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        println!("{:?}", Solution::combine(4, 2));
    }
}