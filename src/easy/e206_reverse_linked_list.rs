// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn _reverse_list(
            curr: Option<Box<ListNode>>,
            prev: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match curr {
                Some(mut node) => {
                    let new_node = node.next.take();
                    node.next = prev;
                    _reverse_list(new_node, Some(node))
                }
                None => prev,
            }
        }
        _reverse_list(head, None)
    }
}
