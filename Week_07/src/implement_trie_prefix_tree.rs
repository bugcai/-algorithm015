#[derive(Default)]
struct Trie {
    root: TrieNode,
}

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Self::default()
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for i in word.chars().map(Self::index_for) {
            node = node.children[i].get_or_insert_with(Box::<TrieNode>::default);
        }
        node.is_word = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        self.get_node(&word).map_or(false, |w| w.is_word)
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        self.get_node(&prefix).is_some()
    }

    fn index_for(c: char) -> usize {
        (c as usize) - ('a' as usize)
    }

    fn get_node(&self, word: &str) -> Option<&TrieNode> {
        let mut node = &self.root;
        for i in word.chars().map(Self::index_for) {
            match node.children[i] {
                Some(ref child) => node = child,
                None => return None,
            }
        }
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));  // 返回 true
        assert!(!trie.search("app".to_string()));     // 返回 false
        assert!(trie.starts_with("app".to_string())); // 返回 true
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));     // 返回 true
    }
}