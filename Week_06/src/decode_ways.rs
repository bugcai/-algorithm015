struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes[0] == b'0' { return 0; }
        if bytes.len() == 1 { return 1; }
        let mut dp = vec![0; bytes.len()];
        dp[0] = 1;
        let (first, second) = (bytes[0] - b'0', bytes[1] - b'0');
        match (first, second) {
            (1, 0) | (2, 0) => dp[1] = 1,
            (_, 0) => return 0,
            _ => {
                let two_digit_num = first * 10 + second;
                dp[1] = if two_digit_num > 26 { 1 } else { 2 };
            }
        }
        for i in 2..bytes.len() {
            let (pre, now) = (bytes[i - 1] - b'0', bytes[i] - b'0');
            match (pre, now) {
                (1, 0) | (2, 0) => dp[i] = dp[i - 2],
                (_, 0) => return 0,
                (0, _) => dp[i] = dp[i - 1],
                _ => {
                    let two_digit_num = pre * 10 + now;
                    if two_digit_num == 10 || two_digit_num == 20 {
                        dp[i] = dp[i - 2];
                    } else if two_digit_num > 26 {
                        dp[i] = dp[i - 1];
                    } else {
                        dp[i] = dp[i - 1] + dp[i - 2];
                    }
                }
            }
        }
        dp[bytes.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::num_decodings("0".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::num_decodings("12".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::num_decodings("226".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(1, Solution::num_decodings("10".to_string()));
    }

    #[test]
    fn case5() {
        assert_eq!(1, Solution::num_decodings("2101".to_string()));
    }

    #[test]
    fn case6() {
        assert_eq!(3, Solution::num_decodings("226".to_string()));
    }

    #[test]
    fn case7() {
        assert_eq!(3, Solution::num_decodings("1201234".to_string()));
    }

    #[test]
    fn case8() {
        assert_eq!(0, Solution::num_decodings("10011".to_string()));
    }

    #[test]
    fn case9() {
        assert_eq!(0, Solution::num_decodings("1090".to_string()));
    }

    #[test]
    fn case10() {
        assert_eq!(5, Solution::num_decodings("1123".to_string()));
    }
}