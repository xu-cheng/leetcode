pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let (first, rest) = match strs.split_first() {
            Some(x) => x,
            None => return "".to_string(),
        };

        let mut iters = rest.iter().map(|s| s.chars()).collect::<Vec<_>>();
        first
            .chars()
            .take_while(|&c| iters.iter_mut().all(|i| i.next() == Some(c)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::longest_common_prefix(str_vec![
                "flower", "flow", "flight"
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(str_vec!["dog", "racecar", "car"]),
            "".to_string()
        );
    }
}
