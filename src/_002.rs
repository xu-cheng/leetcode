#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ans = ListNode::new(0);
        let mut cur = &mut ans;
        let mut carry = 0;

        let mut l1p = l1.as_ref();
        let mut l2p = l2.as_ref();

        while l1p.is_some() || l2p.is_some() {
            let l1n = l1p.map(|b| b.as_ref());
            let l2n = l2p.map(|b| b.as_ref());
            let value =
                l1n.map_or(0, |n| n.val) + l2n.map_or(0, |n| n.val) + carry;
            carry = value / 10;
            cur.next = Some(Box::new(ListNode::new(value % 10)));
            cur = cur.next.as_mut().unwrap();
            l1p = l1n.and_then(|n| n.next.as_ref());
            l2p = l2n.and_then(|n| n.next.as_ref());
        }

        if carry > 0 {
            cur.next = Some(Box::new(ListNode::new(carry)));
        }

        ans.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let ans = Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None })),
            })),
        }));

        assert_eq!(Solution::add_two_numbers(l1, l2), ans);
    }
}
