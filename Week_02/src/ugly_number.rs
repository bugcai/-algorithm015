use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let (mut a, mut b, mut c) = (0, 0, 0);
        let mut dp = Vec::with_capacity(n as usize);
        let n = n as usize;
        dp.push(1);
        for _ in 1..n {
            let (n2, n3, n5) = (dp[a] * 2, dp[b] * 3, dp[c] * 5);
            let ugly_number = n2.min(n3).min(n5);
            dp.push(ugly_number);
            if (ugly_number == n2) {
                a += 1;
            }
            if (ugly_number == n3) {
                b += 1;
            }
            if (ugly_number == n5) {
                c += 1;
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(12, Solution::nth_ugly_number(10));
    }
}