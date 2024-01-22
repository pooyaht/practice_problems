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
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_some() {
            format!(
                "{}:{}:{}",
                root.as_ref().unwrap().borrow().val,
                self.serialize(root.as_ref().unwrap().borrow().left.clone()),
                self.serialize(root.as_ref().unwrap().borrow().right.clone())
            )
        } else {
            "".to_string()
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        fn _deserialize<'a>(
            iter: &mut impl Iterator<Item = &'a str>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(val) = iter.next() {
                let val = val.parse::<i32>().ok()?;
                let left = _deserialize(iter);
                let right = _deserialize(iter);
                Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
            } else {
                None
            }
        }
        if data.is_empty() {
            return None;
        }
        let mut iter = data.split(':');
        _deserialize(iter.by_ref())
    }
}
