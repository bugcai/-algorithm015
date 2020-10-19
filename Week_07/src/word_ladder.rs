struct Solution;

use std::collections::HashSet;
use std::mem::swap;

// 双向广度优先搜索
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_list = word_list.into_iter()
            .map(|x| x.chars().collect())
            .collect::<HashSet<Vec<_>>>();
        let mut front = {
            let mut set = HashSet::new();
            set.insert(begin_word.chars().collect::<Vec<_>>());
            set
        };
        let mut back = {
            let mut set = HashSet::new();
            let end_word = end_word.chars().collect::<Vec<_>>();
            if !word_list.contains(&end_word) {
                return 0;
            }
            set.insert(end_word);
            set
        };

        let mut dis = 1;
        let mut seen = HashSet::new();
        while !front.is_empty() {
            dis += 1;
            let mut tmp = HashSet::new();
            for mut word in front.into_iter() {
                seen.insert(word.clone());
                for i in 0..word.len() {
                    let old = word[i];
                    for c in 'a' as u32..='z' as u32 {
                        word[i] = std::char::from_u32(c).unwrap();
                        if !seen.contains(&word) && word_list.contains(&word) {
                            if back.contains(&word) {
                                return dis;
                            }
                            tmp.insert(word.clone());
                        }
                    }
                    word[i] = old;
                }
            }
            front = tmp;
            if back.len() < front.len() {
                swap(&mut front, &mut back);
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::ladder_length("hit".to_string(),
                                              "cog".to_string(),
                                              vec![String::from("hot"), String::from("dot"), String::from("dog"), String::from("lot"), String::from("log"), String::from("cog")]));
    }
}