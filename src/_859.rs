use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn buddy_strings(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut char_counts: HashMap<char, usize> = HashMap::new();
        let mut in_a_not_b: HashMap<char, usize> = HashMap::new();
        let mut in_b_not_a: HashMap<char, usize> = HashMap::new();
        a.chars().zip(b.chars()).for_each(|(c1, c2)| {
            *char_counts.entry(c1).or_insert(0) += 1;
            if c1 != c2 {
                *in_a_not_b.entry(c1).or_insert(0) += 1;
                *in_b_not_a.entry(c2).or_insert(0) += 1;
            }
        });
        if in_a_not_b.is_empty() {
            char_counts.values().any(|&v| v >= 2)
        } else {
            in_a_not_b == in_b_not_a
                && in_a_not_b.values().cloned().sum::<usize>() == 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(Solution::buddy_strings("ab".to_string(), "ba".to_string()));
        assert!(!Solution::buddy_strings("ab".to_string(), "ab".to_string()));
        assert!(Solution::buddy_strings("aa".to_string(), "aa".to_string()));
        assert!(Solution::buddy_strings(
            "aaaaaaabc".to_string(),
            "aaaaaaacb".to_string()
        ));
        assert!(!Solution::buddy_strings("".to_string(), "aa".to_string()));
    }
}
