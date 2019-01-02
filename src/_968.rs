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

#[derive(Debug, PartialEq, Eq)]
enum State {
    Camera,
    NoCamera,
    NoNeed,
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (s, ans) = Self::_min_camera_cover(&root);
        if s == State::NoCamera {
            ans + 1
        } else {
            ans
        }
    }

    fn _min_camera_cover(root: &Option<Rc<RefCell<TreeNode>>>) -> (State, i32) {
        match root.as_ref() {
            Some(node) => {
                let n = node.borrow();
                let (l_s, l_ans) = Self::_min_camera_cover(&n.left);
                let (r_s, r_ans) = Self::_min_camera_cover(&n.right);
                if l_s == State::NoCamera || r_s == State::NoCamera {
                    (State::Camera, l_ans + r_ans + 1)
                } else if l_s == State::Camera || r_s == State::Camera {
                    (State::NoNeed, l_ans + r_ans)
                } else {
                    (State::NoCamera, l_ans + r_ans)
                }
            }
            None => (State::NoNeed, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left,
            right,
        })))
    }

    #[test]
    fn test_solution() {
        assert_eq!(
            Solution::min_camera_cover(make_node(
                make_node(make_node(None, None), make_node(None, None)),
                None
            )),
            1
        );

        assert_eq!(
            Solution::min_camera_cover(make_node(
                make_node(
                    make_node(make_node(None, make_node(None, None)), None),
                    None
                ),
                None
            )),
            2
        );
    }
}
