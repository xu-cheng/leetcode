pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left = Self::binary_search(&nums, target, 0, nums.len(), true);
        if left >= nums.len() || nums[left] != target {
            return vec![-1, -1];
        }
        let right = Self::binary_search(&nums, target, left, nums.len(), false);
        vec![left as i32, right as i32]
    }

    fn binary_search(
        nums: &[i32],
        target: i32,
        mut left: usize,
        mut right: usize,
        return_left_most: bool,
    ) -> usize {
        use std::cmp::Ordering;
        while left < right {
            let mid = (left + right) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => {
                    left = mid + 1;
                }
                Ordering::Greater => {
                    right = mid;
                }
                Ordering::Equal => {
                    if return_left_most {
                        right = mid;
                    } else {
                        left = mid + 1;
                    }
                }
            }
        }
        if return_left_most {
            left
        } else {
            right - 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 5),
            vec![0, 0]
        );
        assert_eq!(
            Solution::search_range(vec![5, 6, 7, 8, 8, 10], 6),
            vec![1, 1]
        );
        assert_eq!(Solution::search_range(vec![0], 0), vec![0, 0]);
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1, 4], 4), vec![1, 1]);
    }
}
