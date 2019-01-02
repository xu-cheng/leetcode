use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solution;

impl Solution {
    pub fn spellchecker(
        wordlist: Vec<String>,
        queries: Vec<String>,
    ) -> Vec<String> {
        let words: HashSet<String> =
            HashSet::from_iter(wordlist.iter().cloned());
        let mut cap: HashMap<String, String> =
            HashMap::with_capacity(wordlist.len());
        let mut vow: HashMap<String, String> =
            HashMap::with_capacity(wordlist.len());
        for w in wordlist {
            let c = w.to_lowercase();
            cap.entry(c.clone()).or_insert(w.clone());
            let v = c
                .chars()
                .map(|x| match x {
                    'a' | 'e' | 'i' | 'o' | 'u' => '*',
                    _ => x,
                })
                .collect();
            vow.entry(v).or_insert(w.clone());
        }
        let mut ans: Vec<String> = Vec::with_capacity(queries.len());
        for q in queries {
            if words.contains(&q) {
                ans.push(q.clone());
                continue;
            }
            let c: String = q.to_lowercase();
            if let Some(a) = cap.get(&c) {
                ans.push(a.clone());
                continue;
            }
            let v: String = c
                .chars()
                .map(|x| match x {
                    'a' | 'e' | 'i' | 'o' | 'u' => '*',
                    _ => x,
                })
                .collect();
            if let Some(a) = vow.get(&v) {
                ans.push(a.clone());
                continue;
            }
            ans.push("".to_string());
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! str_vec {
        ($($x : expr), *) => (vec![$($x.to_string(),)*])
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::spellchecker(
                str_vec!["KiTe", "kite", "hare", "Hare"],
                str_vec![
                    "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear",
                    "keti", "keet", "keto"
                ]
            ),
            str_vec![
                "kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "",
                "KiTe"
            ]
        );
    }
}
