pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let s_len = s_bytes.len();
        let p_len = p_bytes.len();

        let mut mem = vec![false; (s_len + 1) * (p_len + 1)];
        mem[s_len * (p_len + 1) + p_len] = true;
        for i in (0..=s_len).rev() {
            for j in (0..p_len).rev() {
                let first_match = i < s_len
                    && (p_bytes[j] == s_bytes[i] || p_bytes[j] == b'.');
                mem[i * (p_len + 1) + j] =
                    if j + 1 < p_len && p_bytes[j + 1] == b'*' {
                        mem[i * (p_len + 1) + j + 2]
                            || (first_match && mem[(i + 1) * (p_len + 1) + j])
                    } else {
                        first_match && mem[(i + 1) * (p_len + 1) + j + 1]
                    };
            }
        }

        mem[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
        assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
        assert!(!Solution::is_match(
            "mississippi".to_string(),
            "mis*is*p*.".to_string()
        ));
    }
}
