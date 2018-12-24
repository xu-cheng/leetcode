use std::cmp;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_occur_map = vec![-1 as i32; 256];
        let mut ans = 0;
        let mut cur = 0;
        for (i, c) in s.chars().enumerate() {
            let last_occur = last_occur_map[c as usize];
            if last_occur == -1 {
                cur += 1;
            } else {
                ans = cmp::max(ans, cur);
                cur = cmp::min(cur + 1, i as i32 - last_occur);
            }
            last_occur_map[c as usize] = i as i32;
        }
        ans = cmp::max(ans, cur);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("abba".to_string()),
            2
        );
    }
}
