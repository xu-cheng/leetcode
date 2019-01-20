use std::cmp;
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
        if a.is_empty() {
            return 0;
        }
        let mut ans: i32 = 1;
        let mut inc: i32 = 1;
        let mut dec: i32 = 1;
        for i in 1..a.len() {
            match a[i].cmp(&a[i - 1]) {
                Ordering::Equal => {
                    inc = 1;
                    dec = 1;
                }
                Ordering::Greater => {
                    inc = dec + 1;
                    dec = 1;
                }
                Ordering::Less => {
                    dec = inc + 1;
                    inc = 1;
                }
            }
            ans = cmp::max(ans, cmp::max(inc, dec));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9]),
            5
        );
        assert_eq!(Solution::max_turbulence_size(vec![4, 8, 12, 16]), 2);
        assert_eq!(Solution::max_turbulence_size(vec![100]), 1);
    }
}
