use std::cmp::min;

pub struct Solution;

// Ref: https://leetcode.com/problems/triangle/discuss/38730/DP-Solution-for-Triangle

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut values = match triangle.last() {
            Some(last_level) => last_level.clone(),
            None => return 0,
        };
        for (depth, layer) in triangle.iter().enumerate().rev().skip(1) {
            for i in 0..=depth {
                values[i] = min(values[i], values[i + 1]) + layer[i];
            }
        }
        values[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let triangle1 =
            vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle1), 11);

        let triangle2 = vec![vec![-1], vec![2, 3], vec![1, -1, -3]];
        assert_eq!(Solution::minimum_total(triangle2), -1);
    }
}
