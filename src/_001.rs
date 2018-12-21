use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            let complement = target - *num;
            if let Some(idx) = map.get(&complement) {
                return vec![*idx as i32, i as i32];
            }
            map.insert(*num, i);
        }
        panic!("no solution");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
