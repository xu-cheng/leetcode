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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut cur = root.as_mut().cloned();

        while let Some(cur_node) = cur {
            let mut cur_n = cur_node.borrow_mut();
            if let Some(left_node) = cur_n.left.as_mut() {
                let mut left_n = left_node.clone();
                loop {
                    let node = left_n.borrow_mut().right.as_mut().cloned();
                    match node {
                        Some(n) => left_n = n,
                        None => break,
                    }
                }
                left_n.borrow_mut().right = cur_n.right.take();
                cur_n.right = cur_n.left.take();
            }
            cur = cur_n.right.as_mut().cloned();
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
        let mut input = make_node(
            1,
            make_node(2, make_node(3, None, None), make_node(4, None, None)),
            make_node(5, None, make_node(6, None, None)),
        );
        let output = make_node(
            1,
            None,
            make_node(
                2,
                None,
                make_node(
                    3,
                    None,
                    make_node(
                        4,
                        None,
                        make_node(5, None, make_node(6, None, None)),
                    ),
                ),
            ),
        );
        Solution::flatten(&mut input);
        assert_eq!(input, output);
    }
}
