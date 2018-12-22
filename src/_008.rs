pub struct Solution;

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        if str.is_empty() {
            return 0;
        }

        let mut first = false;
        let mut neg = false;
        let mut ans: i32 = 0;

        for c in str.chars() {
            if first {
                match c {
                    '0'..='9' => {
                        let (tmp1, flag1) = ans.overflowing_mul(10);
                        let (tmp2, flag2) = tmp1.overflowing_add(c as i32 - '0' as i32);
                        if flag1 || flag2 {
                            return if neg {
                                i32::min_value()
                            } else {
                                i32::max_value()
                            };
                        }
                        ans = tmp2;
                    }
                    _ => break,
                }
            } else {
                match c {
                    ' ' => continue,
                    '+' => {
                        first = true;
                        neg = false;
                    }
                    '-' => {
                        first = true;
                        neg = true;
                    }
                    '0'..='9' => {
                        first = true;
                        ans = c as i32 - '0' as i32;
                    }
                    _ => {
                        return 0;
                    }
                }
            }
        }

        if neg {
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
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
}
