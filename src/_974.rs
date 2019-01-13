use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let mut prefix_sum = 0;
        let mut mod_map: HashMap<i32, i32> = HashMap::new(); // mapping <prefix_sum % k, count>
        for &x in a.iter() {
            prefix_sum += x;
            let mut mod_value = prefix_sum % k;
            if mod_value < 0 {
                mod_value += k;
            }
            *mod_map.entry(mod_value).or_insert(0) += 1;
        }
        let mut ans: i32 = 0;
        for (&mod_value, &n) in mod_map.iter() {
            if mod_value == 0 {
                ans += n;
            }
            ans += n * (n - 1) / 2;
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
            Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5),
            7
        );
        assert_eq!(Solution::subarrays_div_by_k(vec![5, 5, 5, 5], 5), 10);
        assert_eq!(Solution::subarrays_div_by_k(vec![-1, 2, 9], 2), 2);
        assert_eq!(Solution::subarrays_div_by_k(vec![-1, -9, 4, 0], 9), 2);
    }
}
