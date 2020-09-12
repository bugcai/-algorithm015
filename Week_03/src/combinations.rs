struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn back_track(start: i32, end: i32, k: i32, results: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            if k == 0 {
                results.push(path.clone());
                return;
            }
            for i in start..end + 1 {
                if end - i + 1 >= k {
                    path.push(i);
                    back_track(i + 1, end, k - 1, results, path);
                    path.pop();
                }
            }
        }
        let mut results = vec![];
        back_track(1, n, k, &mut results, &mut vec![]);
        results
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