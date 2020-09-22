struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let mut freqs = vec![0; 26];
        s.as_bytes().iter().for_each(|&b| freqs[(b - b'a') as usize] += 1);
        t.as_bytes().iter().for_each(|&b| freqs[(b - b'a') as usize] -= 1);
        freqs.iter().all(|&f| f == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_anagram(String::from("anagram"), String::from("nagaram")));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_anagram(String::from("rat"), String::from("car")));
    }
}