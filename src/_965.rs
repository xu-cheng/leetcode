use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::_is_unival_tree(&root, None)
    }

    fn _is_unival_tree(
        root: &Option<Rc<RefCell<TreeNode>>>,
        val: Option<i32>,
    ) -> bool {
        match root.as_ref() {
            Some(node) => {
                let n = node.borrow();
                n.val == val.unwrap_or(n.val)
                    && Self::_is_unival_tree(&n.left, Some(n.val))
                    && Self::_is_unival_tree(&n.right, Some(n.val))
            }
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_solution() {
        assert!(Solution::is_unival_tree(None));
        assert!(Solution::is_unival_tree(make_node(
            1,
            make_node(1, make_node(1, None, None), make_node(1, None, None)),
            make_node(1, None, make_node(1, None, None))
        )));
        assert!(!Solution::is_unival_tree(make_node(
            2,
            make_node(2, make_node(5, None, None), make_node(2, None, None)),
            make_node(2, None, None)
        )));
    }
}
