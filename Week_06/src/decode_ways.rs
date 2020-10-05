struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes[0] == b'0' { return 0; }

        let (mut pre, mut cur) = (1, 1);
        for i in 1..bytes.len() {
            let last_one = bytes[i] - b'0';
            let last_two = (bytes[i - 1] - b'0') * 10 + last_one;
            let mut tmp = 0;
            if last_one != 0 { tmp += cur; }
            if last_two >= 10 && last_two <= 26 { tmp += pre; }
            pre = cur;
            cur = tmp;
            if cur == 0 { return 0; }
        }
        cur
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