use std::cmp;

pub struct Solution;

impl Solution {
    pub fn least_ops_express_target(x: i32, target: i32) -> i32 {
        let mut pos: i32 = 0; // target % (x ^ (k+1))
        let mut neg: i32 = 0; // x ^ (k + 1) - target % (x ^ (k + 1))
        let mut k: i32 = 0;
        let mut num: i32 = target;
        while num > 0 {
            let cur = num % x;
            num /= x;
            if k == 0 {
                pos = cur * 2;
                neg = (x - cur) * 2;
            } else {
                let pos_tmp = cmp::min(cur * k + pos, (cur + 1) * k + neg);
                let neg_tmp =
                    cmp::min((x - cur) * k + pos, (x - cur - 1) * k + neg);
                pos = pos_tmp;
                neg = neg_tmp;
            }
            k += 1;
        }
        cmp::min(pos, neg + k) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::least_ops_express_target(3, 19), 5);
        assert_eq!(Solution::least_ops_express_target(5, 501), 8);
        assert_eq!(Solution::least_ops_express_target(100, 100_000_000), 3);
        assert_eq!(Solution::least_ops_express_target(3, 929), 19);
    }
}
