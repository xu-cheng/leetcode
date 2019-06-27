use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();
        for num in &nums {
            *map.entry(*num).or_insert(0) += 1;
        }
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut iter = map.iter();
        while let Some((&num, &count)) = iter.next() {
            if num == 0 {
                if count >= 3 {
                    results.push(vec![0; 3]);
                }
            } else if count >= 2 {
                let complement = -2 * num;
                if map.get(&complement).is_some() {
                    results.push(vec![num, num, complement]);
                }
            }
            let iter2 = iter.clone();
            for (&num2, &_) in iter2 {
                let complement = -(num + num2);
                if complement > num2 && map.get(&complement).is_some() {
                    results.push(vec![num, num2, complement]);
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::*;

    #[test]
    fn test_solution() {
        assert_unsort_eq(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])
                .into_iter()
                .map(|mut v| {
                    v.sort();
                    v
                })
                .collect(),
            vec![vec![-1, 0, 1], vec![-1, -1, 2]],
        );
        assert_unsort_eq(Solution::three_sum(vec![1, 2, -2, -1]), vec![]);
        assert_unsort_eq(Solution::three_sum(vec![0; 10000]), vec![vec![0; 3]]);
    }
}
