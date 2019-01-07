use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut ans: HashSet<i32> = HashSet::new();
        for i in 0.. {
            let xpow = x.pow(i);
            if xpow > bound {
                break;
            }
            for j in 0.. {
                let n = xpow + y.pow(j);
                if n <= bound {
                    ans.insert(n);
                } else {
                    break;
                }
                if y == 0 || y == 1 {
                    break;
                }
            }
            if x == 0 || x == 1 {
                break;
            }
        }
        ans.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::*;

    #[test]
    fn test_solution() {
        assert_unsort_eq(
            Solution::powerful_integers(2, 3, 10),
            vec![2, 3, 4, 5, 7, 9, 10],
        );
        assert_unsort_eq(
            Solution::powerful_integers(3, 5, 15),
            vec![2, 4, 6, 8, 10, 14],
        );
        assert_unsort_eq(
            Solution::powerful_integers(1, 2, 100),
            vec![2, 3, 5, 9, 17, 33, 65],
        );
    }
}
