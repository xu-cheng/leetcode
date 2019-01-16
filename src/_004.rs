use std::cmp;

pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (v1, v2) = if nums1.len() > nums2.len() {
            (&nums2, &nums1)
        } else {
            (&nums1, &nums2)
        };
        let mut imin = 0;
        let mut imax = v1.len();
        let half_len = (v1.len() + v2.len() + 1) / 2;
        while imin <= imax {
            let i = (imin + imax) / 2;
            let j = half_len - i;
            if i < v1.len() && v1[i] < v2[j - 1] {
                imin = i + 1;
            } else if i > 0 && v1[i - 1] > v2[j] {
                imax = i - 1;
            } else {
                let lhs_max = if i == 0 {
                    v2[j - 1]
                } else if j == 0 {
                    v1[i - 1]
                } else {
                    cmp::max(v1[i - 1], v2[j - 1])
                };

                if (v1.len() + v2.len()) % 2 == 1 {
                    return f64::from(lhs_max);
                }

                let rhs_min = if i == v1.len() {
                    v2[j]
                } else if j == v2.len() {
                    v1[i]
                } else {
                    cmp::min(v1[i], v2[j])
                };

                return f64::from(lhs_max + rhs_min) / 2.0;
            }
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_float_approx(
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
            2.0,
        );
        assert_float_approx(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
            2.5,
        );
    }
}
