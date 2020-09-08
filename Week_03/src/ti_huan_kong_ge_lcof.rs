struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        s.split(" ").collect::<Vec<&str>>().join("%20")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        assert_eq!(String::from("We%20are%20happy."), Solution::replace_space(String::from("We are happy.")))
    }
}