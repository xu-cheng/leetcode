pub struct Solution;

impl Solution {
    pub fn pancake_sort(mut a: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        while a.len() > 1 {
            // find the largest element in a, which is equal to len(a)
            let len = a.len() as i32;
            let maxi = a.iter().position(|&x| x == len).unwrap();
            match maxi {
                x if x == a.len() - 1 => {
                    a.pop();
                }
                0 => {
                    ans.push(len);
                    a.reverse();
                    a.pop();
                }
                _ => {
                    ans.append(&mut vec![maxi as i32 + 1, len]);
                    a = a
                        .iter()
                        .skip(maxi + 1)
                        .rev()
                        .chain(a.iter().take(maxi))
                        .cloned()
                        .collect();
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_pancake_sort(a: &[i32], steps: &[i32]) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::from(a);
        for idx in steps {
            let mut tmp: Vec<i32> = Vec::with_capacity(ans.len());
            for &x in ans[..*idx as usize].iter().rev() {
                tmp.push(x);
            }
            for &x in ans[*idx as usize..].iter() {
                tmp.push(x);
            }
            ans = tmp;
        }
        ans
    }

    fn assert_pancake_sort(a: Vec<i32>) {
        let mut sort_a = a.clone();
        sort_a.sort();
        assert_eq!(
            do_pancake_sort(&a, &Solution::pancake_sort(a.clone())),
            sort_a
        );
    }

    #[test]
    fn test_solution() {
        assert_pancake_sort(vec![3, 2, 4, 1]);
        assert_pancake_sort(vec![1, 2, 3]);
    }
}
