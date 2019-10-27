pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut a: Vec<i32>) -> i32 {
        a.sort_by_key(|k| -k);
        for (i, &x) in a.iter().enumerate() {
            if i + 2 < a.len() {
                let y = a[i + 1];
                let z = a[i + 2];
                if y + z > x {
                    return x + y + z;
                }
            } else {
                break;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
        assert_eq!(Solution::largest_perimeter(vec![1, 2, 1]), 0);
        assert_eq!(Solution::largest_perimeter(vec![3, 2, 3, 4]), 10);
        assert_eq!(Solution::largest_perimeter(vec![3, 6, 2, 3]), 8);
    }
}
