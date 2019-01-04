use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freqs: HashMap<i32, usize> = HashMap::new();
        for n in &nums {
            *freqs.entry(*n).or_insert(0) += 1;
        }
        let mut heap =
            BinaryHeap::from_iter(freqs.iter().map(|(n, freq)| (*freq, *n)));
        let mut ans: Vec<i32> = Vec::with_capacity(k as usize);
        for _ in 0..k {
            ans.push(heap.pop().unwrap().1);
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
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
