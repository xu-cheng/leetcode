use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, g: Vec<i32>) -> i32 {
        let g_set: HashSet<i32> = HashSet::from_iter(g.iter().cloned());
        let mut node = head.as_ref();
        let mut inside_components = false;
        let mut ans = 0;
        while node.is_some() {
            let node_ref = node.unwrap().as_ref();
            let inside_g = g_set.contains(&node_ref.val);
            node = node_ref.next.as_ref();

            if inside_components {
                if !inside_g {
                    inside_components = false;
                }
            } else if inside_g {
                inside_components = true;
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_list(input: &[i32]) -> Option<Box<ListNode>> {
        if input.is_empty() {
            None
        } else {
            Some(Box::new(ListNode {
                val: input[0],
                next: make_list(&input[1..]),
            }))
        }
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::num_components(
                make_list(&[0, 1, 2, 3]),
                vec![0, 1, 3]
            ),
            2
        );
        assert_eq!(
            Solution::num_components(
                make_list(&[0, 1, 2, 3, 4]),
                vec![0, 3, 1, 4]
            ),
            2
        );
    }
}
