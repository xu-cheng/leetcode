pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let next_char_iter =
            s.chars().skip(1).map(Some).chain(std::iter::once(None));
        s.chars()
            .zip(next_char_iter)
            .map(|(c, next_c)| match c {
                'M' => 1000,
                'D' => 500,
                'L' => 50,
                'V' => 5,
                'C' => {
                    if next_c == Some('M') || next_c == Some('D') {
                        -100
                    } else {
                        100
                    }
                }
                'X' => {
                    if next_c == Some('C') || next_c == Some('L') {
                        -10
                    } else {
                        10
                    }
                }
                'I' => {
                    if next_c == Some('X') || next_c == Some('V') {
                        -1
                    } else {
                        1
                    }
                }
                _ => unreachable!(),
            })
            .sum()
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
