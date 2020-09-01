use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();

        for s in strs {
            let mut cnt = vec![0; 26];
            s.bytes().for_each(|c| cnt[(c - b'a') as usize] += 1);
            anagrams.entry(cnt).or_insert(Vec::new()).push(s);
        }
        anagrams.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let groups = Solution::group_anagrams(vec![String::from("eat"), String::from("tea"), String::from("tan"), String::from("ate"), String::from("nat"), String::from("bat")]);
        println!("{:?}", groups);
    }
}