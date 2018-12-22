pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut a: i32 = x;
        let mut b: i32 = 0;
        while a > 0 {
            b *= 10;
            b += a % 10;
            a /= 10;
        }
        b == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
    }
}
