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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::get_leaves(&root1) == Self::get_leaves(&root2)
    }

    fn get_leaves(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut ans: Vec<i32> = Vec::new();
        if let Some(node) = root.as_ref().cloned() {
            stack.push(node);
        }
        while let Some(node) = stack.pop() {
            let n = node.borrow();
            if n.left.is_some() || n.right.is_some() {
                if let Some(lhs) = n.left.as_ref().cloned() {
                    stack.push(lhs);
                }
                if let Some(rhs) = n.right.as_ref().cloned() {
                    stack.push(rhs);
                }
            } else {
                ans.push(n.val);
            }
        }
        ans
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
        let tree1 = make_node(
            3,
            make_node(
                5,
                make_node(6, None, None),
                make_node(
                    2,
                    make_node(7, None, None),
                    make_node(4, None, None),
                ),
            ),
            make_node(1, make_node(9, None, None), make_node(8, None, None)),
        );
        let tree2 = make_node(
            3,
            make_node(1, make_node(6, None, None), make_node(7, None, None)),
            make_node(
                5,
                make_node(
                    2,
                    make_node(4, None, None),
                    make_node(9, None, None),
                ),
                make_node(8, None, None),
            ),
        );
        assert!(Solution::leaf_similar(tree1, tree2));
    }
}
