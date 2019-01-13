use std::collections::{BTreeMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn odd_even_jumps(a: Vec<i32>) -> i32 {
        let end_idx = a.len() - 1;
        let mut inv_map: BTreeMap<i32, usize> = BTreeMap::new(); // mapping <value, idx>
        let mut poi_odd: HashSet<usize> = HashSet::new(); // idx can get to the end starting from odd jump
        let mut poi_even: HashSet<usize> = HashSet::new(); // idx can get to the end starting from even jump
        inv_map.insert(a[end_idx], end_idx);
        for (i, &v) in a.iter().enumerate().rev().skip(1) {
            // odd jump: jump to the smallest higher value
            if let Some((_, &p)) = inv_map.range(v..).next() {
                if poi_even.contains(&p) || p == end_idx {
                    poi_odd.insert(i);
                }
            }
            // even jump: jump to the largest lower value
            if let Some((_, &p)) = inv_map.range(..=v).rev().next() {
                if poi_odd.contains(&p) || p == end_idx {
                    poi_even.insert(i);
                }
            }

            inv_map.insert(v, i);
        }
        poi_odd.len() as i32 + 1 // + 1 because starting from end_idx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::odd_even_jumps(vec![10, 13, 12, 14, 15]), 2);
        assert_eq!(Solution::odd_even_jumps(vec![2, 3, 1, 1, 4]), 3);
        assert_eq!(Solution::odd_even_jumps(vec![5, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::odd_even_jumps(vec![1, 2, 3, 2, 1, 4, 4, 5]), 6);
    }
}
