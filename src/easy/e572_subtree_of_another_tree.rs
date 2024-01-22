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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        if sub_root.is_none() {
            return true;
        }

        if Self::is_identical(&root, &sub_root) {
            return true;
        }
        let left_subtree = Self::is_subtree(
            root.as_ref().unwrap().borrow().left.clone(),
            sub_root.clone(),
        );
        let right_subtree = Self::is_subtree(root.unwrap().borrow().right.clone(), sub_root);
        left_subtree || right_subtree
    }
    pub fn is_identical(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(root), Some(sub_root)) => {
                if root.borrow().val == sub_root.borrow().val {
                    return Self::is_identical(&root.borrow().left, &sub_root.borrow().left)
                        && Self::is_identical(&root.borrow().right, &sub_root.borrow().right);
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
