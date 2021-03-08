use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap: BinaryHeap<(i32, Vec<i32>)> = points
            .into_iter()
            .map(|v| (-(v[0] * v[0] + v[1] * v[1]), v))
            .collect();
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(k as usize);
        while ans.len() < k as usize && !heap.is_empty() {
            ans.push(heap.pop().unwrap().1);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::*;

    #[test]
    fn test_solution() {
        assert_unsort_eq(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]],
        );
        assert_unsort_eq(
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]],
        );
    }
}
