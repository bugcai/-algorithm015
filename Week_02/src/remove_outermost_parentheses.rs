struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::new();
        let mut opened = 0;
        for ch in s.chars() {
            if ch == ')' {
                opened -= 1;
            }
            if opened > 0 {
                result.push(ch);
            }
            if ch == '(' {
                opened += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(String::from("()()()"), Solution::remove_outer_parentheses(String::from("(()())(())")));
    }

    #[test]
    fn case2() {
        assert_eq!(String::from("()()()()(())"), Solution::remove_outer_parentheses(String::from("(()())(())(()(()))")));
    }

    #[test]
    fn case3() {
        assert_eq!(String::from(""), Solution::remove_outer_parentheses(String::from("()()")));
    }
}