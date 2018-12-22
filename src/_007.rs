pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num: i32 = x.abs();
        let mut ans: i32 = 0;
        while num > 0 {
            let (tmp1, flag) = ans.overflowing_mul(10);
            if flag {
                return 0;
            }
            let (tmp2, flag) = tmp1.overflowing_add(num % 10);
            if flag {
                return 0;
            }
            ans = tmp2;
            num /= 10;
        }
        if x < 0 {
            ans = -ans;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
    }
}
