pub struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut result = String::new();
        for (divisor, symbol) in [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ]
        .iter()
        {
            let quotient = num / divisor;
            if quotient > 0 {
                result.push_str(&symbol.repeat(quotient as usize));
            }
            num %= divisor;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
