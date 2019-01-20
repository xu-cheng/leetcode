pub struct Solution;

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![0; a.len()];
        let mut lhs_idx = 0;
        let mut rhs_idx = a.len() - 1;
        for i in (0..a.len()).rev() {
            if a[lhs_idx].abs() > a[rhs_idx].abs() {
                ans[i] = a[lhs_idx] * a[lhs_idx];
                lhs_idx += 1;
            } else {
                ans[i] = a[rhs_idx] * a[rhs_idx];
                rhs_idx -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::sorted_squares(vec![-4,-1,0,3,10]), vec![0,1,9,16,100]);
        assert_eq!(Solution::sorted_squares(vec![-7,-3,2,3,11]), vec![4,9,9,49,121]);
    }
}
