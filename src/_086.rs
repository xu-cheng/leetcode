use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct List {
    pub head: Option<Box<ListNode>>,
    pub tail: *mut ListNode,
}

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    fn append(&mut self, val: i32) {
        let mut node = Box::new(ListNode::new(val));
        let ptr: *mut _ = &mut *node;
        if self.head.is_some() {
            unsafe {
                (*self.tail).next = Some(node);
            }
        } else {
            self.head = Some(node);
        }
        self.tail = ptr;
    }

    fn extend(&mut self, other: List) {
        if self.head.is_some() && other.head.is_some() {
            unsafe {
                (*self.tail).next = other.head;
            }
            self.tail = other.tail;
        } else if self.head.is_none() && other.head.is_some() {
            self.head = other.head;
            self.tail = other.tail;
        }
    }
}
pub struct Solution;

impl Solution {
    pub fn partition(
        head: Option<Box<ListNode>>,
        x: i32,
    ) -> Option<Box<ListNode>> {
        let mut less = List::new();
        let mut greater_or_equal = List::new();

        let mut ptr = head.as_ref();
        while let Some(node) = ptr {
            let n = node.as_ref();
            match n.val.cmp(&x) {
                Ordering::Less => less.append(n.val),
                Ordering::Equal | Ordering::Greater => {
                    greater_or_equal.append(n.val)
                }
            }
            ptr = n.next.as_ref();
        }

        less.extend(greater_or_equal);
        less.head
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
            Solution::partition(make_list(&[1, 4, 3, 2, 5, 2]), 3),
            make_list(&[1, 2, 2, 4, 3, 5])
        );
    }
}
