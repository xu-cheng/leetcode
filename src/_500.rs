pub struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        words
            .into_iter()
            .filter(|s| Self::can_be_typed(&s))
            .collect()
    }

    fn can_be_typed(s: &str) -> bool {
        let mut last_row = 0;
        for c in s.chars() {
            let row = match c.to_ascii_lowercase() {
                'q' | 'w' | 'e' | 'r' | 't' | 'y' | 'u' | 'i' | 'o' | 'p' => 1,
                'a' | 's' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' => 2,
                'z' | 'x' | 'c' | 'v' | 'b' | 'n' | 'm' => 3,
                _ => {
                    panic!("invalid char!");
                }
            };
            if last_row == 0 {
                last_row = row;
            } else if last_row != row {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::find_words(str_vec!["Hello", "Alaska", "Dad", "Peace"]),
            str_vec!["Alaska", "Dad"]
        );
    }
}
