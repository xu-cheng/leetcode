pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut iter = s.chars().peekable();
        let mut ans: i32 = 0;
        while let Some(c) = iter.next() {
            ans += match c {
                'M' => 1000,
                'D' => 500,
                'L' => 50,
                'V' => 5,
                'C' => {
                    let next_c = iter.peek();
                    if next_c == Some(&&'M') || next_c == Some(&&'D') {
                        -100
                    } else {
                        100
                    }
                }
                'X' => {
                    let next_c = iter.peek();
                    if next_c == Some(&&'C') || next_c == Some(&&'L') {
                        -10
                    } else {
                        10
                    }
                }
                'I' => {
                    let next_c = iter.peek();
                    if next_c == Some(&&'X') || next_c == Some(&&'V') {
                        -1
                    } else {
                        1
                    }
                }
                _ => unreachable!(),
            };
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
