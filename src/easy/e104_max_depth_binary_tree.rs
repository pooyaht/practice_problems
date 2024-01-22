//Definition for a binary tree node.
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
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn _max_depth(root: Option<Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
            if let Some(node) = root {
                _max_depth(node.borrow().left.clone(), depth + 1)
                    .max(_max_depth(node.borrow().right.clone(), depth + 1))
            } else {
                depth
            }
        }
        _max_depth(root, 0)
    }
}
