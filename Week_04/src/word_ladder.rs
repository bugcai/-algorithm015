struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::{HashMap, HashSet, VecDeque};
        let mut all_combo_dict: HashMap<String, Vec<String>> = HashMap::new();
        for word in word_list.iter() {
            for i in 0..word.len() {
                let s = word.as_str();
                let key = (&s[0..i]).to_string() + "*" + &s[i + 1..];
                all_combo_dict.entry(key).or_insert(vec![]).push(s.to_string());
            }
        }
        let mut visited: HashSet<String> = HashSet::new();
        let mut q: VecDeque<(String, i32)> = VecDeque::new();
        visited.insert(begin_word.clone());
        q.push_back((begin_word, 1));
        while let Some(front) = q.pop_front() {
            let word = front.0.as_str();
            for i in 0..word.len() {
                let new_word = (&word[0..i]).to_string() + "*" + &word[i + 1..];
                if let Some(v) = all_combo_dict.get(&new_word) {
                    for adjacent_word in v.iter() {
                        if *adjacent_word == end_word {
                            return front.1 + 1;
                        }
                        if !visited.contains(adjacent_word) {
                            visited.insert(adjacent_word.to_string());
                            q.push_back((adjacent_word.to_string(), front.1 + 1));
                        }
                    }
                }
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