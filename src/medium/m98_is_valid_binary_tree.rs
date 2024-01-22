// Definition for a binary tree node.
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn _is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>, min_val: i64, max_val: i64) -> bool {
            if root.is_none() {
                return true;
            }
            let root = root.unwrap();
            if root.borrow().val as i64 <= min_val || root.borrow().val as i64 >= max_val {
                return false;
            }
            let result = _is_valid_bst(
                root.borrow().left.clone(),
                min_val,
                root.borrow().val as i64,
            ) && _is_valid_bst(
                root.borrow().right.clone(),
                root.borrow().val as i64,
                max_val,
            );
            return result;
        }

        _is_valid_bst(root, i64::MIN, i64::MAX)
    }
}
