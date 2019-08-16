pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        (n > 0) && ((n & (n - 1)) == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(Solution::is_power_of_two(1));
        assert!(Solution::is_power_of_two(16));
        assert!(!Solution::is_power_of_two(218));
        assert!(!Solution::is_power_of_two(-2_147_483_648));
    }
}
