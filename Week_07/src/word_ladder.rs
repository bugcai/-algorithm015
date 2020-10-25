struct Solution;

use std::mem::swap;
use std::collections::{VecDeque, HashMap};

// 双向广度优先搜索
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        // 字典 map，并记录对应单词是否访问过
        // 0 - 表示未被访问过
        // 1 - 表示在前向搜索时访问过
        // 2 - 表示在后向搜索时访问过
        // 3 - 表示前后向均访问过，可结束搜索
        let mut word_map: HashMap<Vec<u8>, u8> = word_list.iter()
            .map(|w| (Vec::from(w.as_bytes()), if *w == end_word { 2 } else { 0 }))
            .collect();
        // 前向双端队列，从 begin_word 查找 end_word
        let mut front = {
            let mut deq = VecDeque::new();
            deq.push_back(Vec::from(begin_word.as_bytes()));
            deq
        };
        // 后向双端队列
        let mut back = {
            let mut deq = VecDeque::new();
            let end_word = Vec::from(end_word.as_bytes());
            if !word_map.contains_key(&end_word) {
                return 0;
            }
            deq.push_back(end_word);
            deq
        };

        // 访问模式
        // 1 - 表示前向搜索；2 - 表示后向搜索
        let mut mode: u8 = 0b01;
        let mut dis = 1;
        while !front.is_empty() {
            dis += 1;
            for _ in 0..front.len() {
                let mut word = front.pop_front().unwrap();
                for i in 0..word.len() {
                    let old = word[i];
                    for c in b'a'..=b'z' {
                        word[i] = c;
                        if let Some(flag) = word_map.get_mut(&word) {
                            if *flag == mode { continue; }
                            *flag += mode;
                            if *flag == 0b11 { return dis; }
                            front.push_back(word.clone());
                        }
                    }
                    word[i] = old;
                }
            }
            if back.len() < front.len() {
                swap(&mut front, &mut back);
                mode ^= 0b11;
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