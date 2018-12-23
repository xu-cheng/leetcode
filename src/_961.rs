use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut seen: HashSet<i32> = HashSet::new();
        for n in a {
            if !seen.insert(n) {
                return n;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::repeated_n_times(vec![1, 2, 3, 3]), 3);
        assert_eq!(Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
        assert_eq!(Solution::repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
