pub struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut ans: i32 = 0;
        for c in s.chars() {
            ans *= 26;
            ans += c as i32 - 'A' as i32 + 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }
}
