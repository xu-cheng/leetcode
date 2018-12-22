pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let n = s.len() as i32;
        let cycle = 2 * num_rows - 2;
        let mut ans = String::with_capacity(s.len());
        for i in 0..num_rows {
            let mut j: i32 = 0;
            while i + j < n {
                ans.push(s.chars().nth((i + j) as usize).unwrap());
                if i != 0 && i != num_rows - 1 && j + cycle - i < n {
                    ans.push(s.chars().nth((j + cycle - i) as usize).unwrap());
                }
                j += cycle;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::convert("AB".to_string(), 1), "AB".to_string());
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR".to_string()
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI".to_string()
        );
    }
}
