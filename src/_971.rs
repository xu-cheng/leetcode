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
    pub fn flip_match_voyage(
        root: Option<Rc<RefCell<TreeNode>>>,
        voyage: Vec<i32>,
    ) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        if Self::preorder_traversal(&root, &voyage, &mut 0, &mut ans) {
            ans
        } else {
            vec![-1]
        }
    }

    fn preorder_traversal(
        root: &Option<Rc<RefCell<TreeNode>>>,
        voyage: &[i32],
        voyage_idx: &mut usize,
        ans: &mut Vec<i32>,
    ) -> bool {
        match root.as_ref() {
            None => true,
            Some(node) => {
                let n = node.borrow();
                if n.val != voyage[*voyage_idx] {
                    return false;
                }
                *voyage_idx += 1;
                if n.left
                    .as_ref()
                    .map(|x| x.borrow().val != voyage[*voyage_idx])
                    .unwrap_or(false)
                {
                    ans.push(n.val);
                    Self::preorder_traversal(&n.right, voyage, voyage_idx, ans)
                        && Self::preorder_traversal(
                            &n.left, voyage, voyage_idx, ans,
                        )
                } else {
                    Self::preorder_traversal(&n.left, voyage, voyage_idx, ans)
                        && Self::preorder_traversal(
                            &n.right, voyage, voyage_idx, ans,
                        )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::*;

    fn make_node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_solution() {
        assert_unsort_eq(
            Solution::flip_match_voyage(
                make_node(1, make_node(2, None, None), None),
                vec![2, 1],
            ),
            vec![-1],
        );
        assert_unsort_eq(
            Solution::flip_match_voyage(
                make_node(
                    1,
                    make_node(2, None, None),
                    make_node(3, None, None),
                ),
                vec![1, 3, 2],
            ),
            vec![1],
        );
        assert_unsort_eq(
            Solution::flip_match_voyage(
                make_node(
                    1,
                    make_node(2, None, None),
                    make_node(3, None, None),
                ),
                vec![1, 2, 3],
            ),
            vec![],
        );
    }
}
