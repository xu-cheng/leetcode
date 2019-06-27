pub struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let len = s.len();
        'outer: for i in 1..=(len / 2) {
            if len % i != 0 {
                continue;
            }
            let first = &s[..i];
            let mut rest = &s[i..];
            while !rest.is_empty() {
                if first != &rest[..i] {
                    continue 'outer;
                }
                rest = &rest[i..];
            }
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(!Solution::repeated_substring_pattern("a".to_string()));
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
        assert!(Solution::repeated_substring_pattern(
            "abcabcabc".to_string()
        ));
        assert!(Solution::repeated_substring_pattern(
            "ababcababc".to_string()
        ));
        assert!(!Solution::repeated_substring_pattern(
            "ababcabab".to_string()
        ));
    }
}
