pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor: i32 = 0;
        for n in &nums {
            xor ^= *n;
        }
        let lsb = xor & -xor;
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        for n in &nums {
            if (*n & lsb) == 0 {
                a ^= *n;
            } else {
                b ^= *n;
            }
        }
        vec![a, b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_unsort_eq(mut a: Vec<i32>, mut b: Vec<i32>) {
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }

    #[test]
    fn test_solution() {
        assert_unsort_eq(
            Solution::single_number(vec![1, 2, 1, 3, 2, 5]),
            vec![3, 5],
        );
        assert_unsort_eq(Solution::single_number(vec![0, 1, 2, 2]), vec![0, 1]);
        assert_unsort_eq(
            Solution::single_number(vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3]),
            vec![4, 5],
        );
    }
}
