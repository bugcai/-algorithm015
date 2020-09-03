use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let fizz_buzz_dict: BTreeMap<i32, &str> = [(3, "Fizz"), (5, "Buzz")].iter().cloned().collect();
        let mut res = Vec::new();
        for i in 1..n + 1 {
            let mut num_or_str = String::new();
            for (k, &str) in fizz_buzz_dict.iter() {
                if i % k == 0 {
                    num_or_str = format!("{}{}", num_or_str, str);
                }
            }
            if num_or_str.len() == 0 {
                num_or_str = format!("{}", i);
            }
            res.push(num_or_str.to_string());
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(vec!["1",
                        "2",
                        "Fizz",
                        "4",
                        "Buzz",
                        "Fizz",
                        "7",
                        "8",
                        "Fizz",
                        "Buzz",
                        "11",
                        "Fizz",
                        "13",
                        "14",
                        "FizzBuzz"],
                   Solution::fizz_buzz(15));
    }
}