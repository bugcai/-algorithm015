use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let mut p2i = HashMap::new();
        let mut s2i = HashMap::new();
        let mut s_iter = str.split(" ");
        let mut p_iter = pattern.chars();
        let mut i = 0;
        loop {
            match (s_iter.next(), p_iter.next()) {
                (Some(sub), Some(ch)) => {
                    if p2i.insert(ch, i) != s2i.insert(sub, i) {
                        return false;
                    }
                },
                (None, None) => break,
                _ => return false,
            }
            i += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::word_pattern(String::from("abba"), String::from("dog cat cat dog")));
    }

    #[test]
    fn case2() {
        assert!(!Solution::word_pattern(String::from("abba"), String::from("dog cat cat fish")));
    }

    #[test]
    fn case3() {
        assert!(!Solution::word_pattern(String::from("aaaa"), String::from("dog cat cat dog")));
    }

    #[test]
    fn case4() {
        assert!(!Solution::word_pattern(String::from("abba"), String::from("dog dog dog dog")));
    }
}