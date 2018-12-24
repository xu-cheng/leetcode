use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let mut seen: HashMap<(Vec<i32>, i32), Vec<Vec<i32>>> = HashMap::new();
        for (i, p1) in points.iter().enumerate() {
            for p2 in points.iter().skip(i + 1) {
                let center = vec![p1[0] + p2[0], p1[1] + p2[1]];
                let radius =
                    ((p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2)) as i32;
                let key = (center, radius);
                match seen.get_mut(&key) {
                    Some(value) => {
                        value.push(p1.clone());
                    }
                    None => {
                        seen.insert(key, vec![p1.clone()]);
                    }
                }
            }
        }

        let mut ans: Option<f64> = None;

        for ((center, _radius), ps) in seen {
            for (i, p1) in ps.iter().enumerate() {
                for p2 in ps.iter().skip(i + 1) {
                    let x1 = (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2);
                    let x2 = (p1[0] + p2[0] - center[0]).pow(2)
                        + (p1[1] + p2[1] - center[1]).pow(2);
                    let area = (x1 as f64).sqrt() * (x2 as f64).sqrt();
                    ans = Some(match ans {
                        Some(value) => {
                            if area > value {
                                value
                            } else {
                                area
                            }
                        }
                        None => area,
                    });
                }
            }
        }

        ans.unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_float_approx(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5);
    }

    #[test]
    fn test_solution() {
        assert_float_approx(
            Solution::min_area_free_rect(vec![
                vec![1, 2],
                vec![2, 1],
                vec![1, 0],
                vec![0, 1],
            ]),
            2.0,
        );
        assert_float_approx(
            Solution::min_area_free_rect(vec![
                vec![0, 1],
                vec![2, 1],
                vec![1, 1],
                vec![1, 0],
                vec![2, 0],
            ]),
            1.0,
        );
        assert_float_approx(
            Solution::min_area_free_rect(vec![
                vec![0, 3],
                vec![1, 2],
                vec![3, 1],
                vec![1, 3],
                vec![2, 1],
            ]),
            0.0,
        );
        assert_float_approx(
            Solution::min_area_free_rect(vec![
                vec![3, 1],
                vec![1, 1],
                vec![0, 1],
                vec![2, 1],
                vec![3, 3],
                vec![3, 2],
                vec![0, 2],
                vec![2, 3],
            ]),
            2.0,
        );
    }
}
