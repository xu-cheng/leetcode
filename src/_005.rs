use std::cmp;

pub struct Solution1;
pub struct Solution2;

impl Solution1 {
    pub fn longest_palindrome(s: String) -> String {
        let s_bytes = s.as_bytes();
        let len = s_bytes.len();
        let mut ans = "".to_string();
        for mid in 0..len {
            let mut i: usize;
            // search palindrome s[mid-i..=mid+i]
            i = 1;
            while mid >= i
                && mid + i < len
                && s_bytes[mid - i] == s_bytes[mid + i]
            {
                i += 1;
            }
            i -= 1;
            if ans.len() < 2 * i + 1 {
                ans = s[mid - i..=mid + i].to_string();
            }
            // search palindrome s[mid-i..=mid+i+1]
            i = 0;
            while mid >= i
                && mid + i + 1 < len
                && s_bytes[mid - i] == s_bytes[mid + i + 1]
            {
                i += 1;
            }
            if i > 0 && ans.len() < 2 * i {
                i -= 1;
                ans = s[mid - i..=mid + i + 1].to_string();
            }
        }
        ans
    }
}

impl Solution2 {
    pub fn longest_palindrome(s: String) -> String {
        let t = Self::preprocess(&s);
        let mut p = vec![0; t.len()];
        let mut c = 0;
        let mut r = 0;
        for i in 1..t.len() - 1 {
            if r > i {
                let i_mirror = 2 * c - i;
                p[i] = cmp::min(r - i, p[i_mirror]);
            }
            while t.as_bytes()[i + 1 + p[i]] == t.as_bytes()[i - 1 - p[i]] {
                p[i] += 1;
            }
            if i + p[i] > r {
                c = i;
                r = i + p[i];
            }
        }
        let mut max_len = 0;
        let mut max_c = 0;
        for (i, len) in p.iter().enumerate().take(p.len() - 1).skip(1) {
            if *len > max_len {
                max_c = i;
                max_len = *len;
            }
        }
        let start = (max_c - 1 - max_len) / 2;
        let end = start + max_len;
        s[start..end].to_string()
    }

    fn preprocess(s: &str) -> String {
        if s.is_empty() {
            return "^$".to_string();
        }
        let mut out = String::from("^");
        for c in s.chars() {
            out.push('#');
            out.push(c);
        }
        out.push_str("#$");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1() {
        let ans1 = Solution1::longest_palindrome("babad".to_string());
        assert!(ans1 == "bab" || ans1 == "aba");
        assert_eq!(
            Solution1::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| {
            Solution1::longest_palindrome(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            )
        });
    }

    #[test]
    fn test_solution2() {
        let ans1 = Solution2::longest_palindrome("babad".to_string());
        assert!(ans1 == "bab" || ans1 == "aba");
        assert_eq!(
            Solution2::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[bench]
    fn bench_solution2(b: &mut Bencher) {
        b.iter(|| {
            Solution2::longest_palindrome(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            )
        });
    }
}
