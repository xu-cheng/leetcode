use std::cmp;

pub struct Solution1;
pub struct Solution2;

impl Solution1 {
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let mut array: Vec<(i32, usize)> = Vec::with_capacity(a.len());
        for (i, n) in a.iter().enumerate() {
            array.push((*n, i));
        }
        array.sort_by_key(|x| x.0);
        let mut ans: i32 = 0;
        let mut imin = a.len();
        for (_n, i) in array {
            if i > imin {
                ans = cmp::max(ans, (i - imin) as i32);
            }
            imin = cmp::min(imin, i);
        }
        ans
    }
}

impl Solution2 {
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::with_capacity(a.len());
        let mut ans: i32 = 0;
        for (i, n) in a.iter().enumerate() {
            if stack.is_empty() || a[stack[stack.len() - 1]] > *n {
                stack.push(i);
            }
        }

        for (i, n) in a.iter().enumerate().rev() {
            while !stack.is_empty() && a[stack[stack.len() - 1]] <= *n {
                let x = stack.pop().unwrap();
                ans = cmp::max(ans, (i - x) as i32);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1() {
        assert_eq!(Solution1::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(
            Solution1::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]),
            7
        );
        assert_eq!(Solution1::max_width_ramp(vec![3, 2, 1]), 0);
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| {
            Solution1::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1])
        });
    }

    #[test]
    fn test_solution2() {
        assert_eq!(Solution2::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
        assert_eq!(
            Solution2::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]),
            7
        );
        assert_eq!(Solution2::max_width_ramp(vec![3, 2, 1]), 0);
    }

    #[bench]
    fn bench_solution2(b: &mut Bencher) {
        b.iter(|| {
            Solution2::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1])
        });
    }
}
