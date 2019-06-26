use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut result: i32 = 0;
        while left < right {
            let height_l = height[left];
            let height_r = height[right];
            let width = (right - left) as i32;
            result = max(result, min(height_l, height_r) * width);
            if height_l < height_r {
                left += 1;
            } else {
                right -= 1;
            };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
