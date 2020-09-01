struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let (mut freq_s, mut freq_t) = (vec![0; 26], vec![0; 26]);
        s.bytes().for_each(|ch| freq_s[ch as usize - b'a' as usize] += 1);
        t.bytes().for_each(|ch| freq_t[ch as usize - b'a' as usize] += 1);
        freq_s == freq_t
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