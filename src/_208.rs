use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    children: HashMap<char, Trie>,
    end_str: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            end_str: false,
        }
    }
    pub fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.chars() {
            cur = cur.children.entry(c).or_default();
        }
        cur.end_str = true;
    }
    fn find_prefix(&self, prefix: &str) -> Option<&Trie> {
        let mut cur = self;
        for c in prefix.chars() {
            cur = cur.children.get(&c)?;
        }
        Some(cur)
    }
    pub fn search(&self, word: String) -> bool {
        self.find_prefix(&word).map_or(false, |n| n.end_str)
    }
    pub fn starts_with(&self, prefix: String) -> bool {
        self.find_prefix(&prefix).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        assert!(trie.search("apple".to_string()));
        assert!(!trie.search("app".to_string()));
        assert!(trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        assert!(trie.search("app".to_string()));
    }
}
