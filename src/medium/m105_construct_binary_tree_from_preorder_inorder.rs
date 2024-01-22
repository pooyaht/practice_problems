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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn _build_tree(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if preorder.is_empty() || inorder.is_empty() {
                None
            } else {
                let root_val = preorder[0];
                let mut iter = inorder.split(|n| *n == root_val);

                let left = iter.next().unwrap();
                let right = iter.next().unwrap();

                let mut root = TreeNode::new(root_val);

                root.left = _build_tree(&preorder[1..left.len() + 1], left);
                root.right = _build_tree(&preorder[left.len() + 1..], right);

                Some(Rc::new(RefCell::new(root)))
            }
        }
        _build_tree(&preorder, &inorder)
    }
}
