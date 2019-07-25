use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, Trie>,
    end_str: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            end_str: false,
        }
    }
    fn insert(&mut self, word: &str) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.children.entry(c).or_default();
        }
        cur.end_str = true;
    }
    fn search(&self, word: &str) -> bool {
        let mut cur = self;
        for c in word.chars() {
            cur = match cur.children.get(&c) {
                Some(n) => n,
                None => return false,
            };
        }
        cur.end_str
    }
}

#[derive(Default)]
pub struct MagicDictionary {
    trie: Trie,
}

impl MagicDictionary {
    pub fn new() -> Self {
        Self { trie: Trie::new() }
    }
    pub fn build_dict(&mut self, dict: Vec<String>) {
        for w in &dict {
            self.trie.insert(w);
        }
    }
    pub fn search(&self, word: String) -> bool {
        let mut cur = &self.trie;
        let mut iter = word.chars();
        while let Some(c) = iter.next() {
            for (&trie_c, sub_trie) in &cur.children {
                if c == trie_c {
                    cur = sub_trie;
                } else if sub_trie.search(iter.as_str()) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(str_vec!["hello", "leetcode"]);
        assert!(!dict.search("hello".to_string()));
        assert!(dict.search("hhllo".to_string()));
        assert!(!dict.search("hell".to_string()));
        assert!(!dict.search("leetcoded".to_string()));
    }
}
