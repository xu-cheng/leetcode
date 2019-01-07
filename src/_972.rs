use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Sub;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[derive(Debug)]
struct Rational(i64, i64);

impl Rational {
    fn new(p: i64, q: i64) -> Rational {
        let mut x = Rational(p, q);
        x.simplify();
        x
    }

    fn reciprocal(&self) -> Rational {
        if self.0 == 0 {
            panic!("cannot compute the reciprocal for 0");
        } else {
            Rational(self.1, self.0)
        }
    }

    fn simplify(&mut self) -> &Rational {
        if self.0 == 0 {
            self.1 = 1;
        } else {
            let x = gcd(self.0, self.1);
            self.0 /= x;
            self.1 /= x;
        }
        self
    }
}

impl From<&str> for Rational {
    fn from(s: &str) -> Rational {
        let (int_s, dot_s, repeat_s) = match s.find('.') {
            None => (s, "", ""),
            Some(dot_p) => match s.find('(') {
                None => (&s[..dot_p], &s[dot_p + 1..], ""),
                Some(repeat_p) => (
                    &s[..dot_p],
                    &s[dot_p + 1..repeat_p],
                    &s[repeat_p + 1..s.len() - 1],
                ),
            },
        };
        let mut ans = Rational::new(int_s.parse().unwrap_or(0), 1);
        ans += Rational::new(
            dot_s.parse().unwrap_or(0),
            i64::pow(10, dot_s.len() as u32),
        );
        let a0 = Rational::new(
            repeat_s.parse().unwrap_or(0),
            i64::pow(10, (dot_s.len() + repeat_s.len()) as u32),
        );
        let q = Rational::new(1, i64::pow(10, repeat_s.len() as u32));
        if a0.0 != 0 {
            ans += a0 * (Rational::new(1, 1) - q).reciprocal();
        }
        ans
    }
}

impl Add for Rational {
    type Output = Rational;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, other: Rational) -> Rational {
        Rational::new(self.0 * other.1 + self.1 * other.0, self.1 * other.1)
    }
}

impl AddAssign for Rational {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, other: Rational) {
        *self = Rational::new(
            self.0 * other.1 + self.1 * other.0,
            self.1 * other.1,
        );
    }
}

impl Mul for Rational {
    type Output = Rational;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, other: Rational) -> Rational {
        Rational::new(self.0 * other.0, self.1 * other.1)
    }
}

impl Sub for Rational {
    type Output = Rational;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, other: Rational) -> Rational {
        Rational::new(self.0 * other.1 - self.1 * other.0, self.1 * other.1)
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Rational) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}

pub struct Solution;

impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        Rational::from(s.as_str()) == Rational::from(t.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert!(Solution::is_rational_equal(
            "0.(52)".to_string(),
            "0.5(25)".to_string()
        ));
        assert!(Solution::is_rational_equal(
            "0.1666(6)".to_string(),
            "0.166(66)".to_string()
        ));
        assert!(Solution::is_rational_equal(
            "0.9(9)".to_string(),
            "1.".to_string()
        ));
        assert!(Solution::is_rational_equal(
            ".99(99)".to_string(),
            "1.".to_string()
        ));
        assert!(!Solution::is_rational_equal(
            "40.2579".to_string(),
            "39.9999".to_string()
        ));
    }
}
