pub struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        if n == 1 {
            return vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        }

        let mut ans = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for _ in 1..n {
            let mut tmp: Vec<i32> = Vec::new();
            for x in &ans {
                let a = x % 10;
                if a >= k {
                    tmp.push(x * 10 + a - k);
                }
                if a + k <= 9 {
                    tmp.push(x * 10 + a + k);
                }
            }
            ans = tmp;
        }
        ans.sort();
        ans.dedup();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::nums_same_consec_diff(3, 7),
            vec![181, 292, 707, 818, 929],
        );
        assert_eq!(
            Solution::nums_same_consec_diff(2, 1),
            vec![
                10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89,
                98,
            ],
        );
        assert_eq!(
            Solution::nums_same_consec_diff(1, 0),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        );
        assert_eq!(
            Solution::nums_same_consec_diff(2, 0),
            vec![11, 22, 33, 44, 55, 66, 77, 88, 99]
        );
    }
}
