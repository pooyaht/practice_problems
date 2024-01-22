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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut fast = head.clone();
        let mut slow = head.as_mut();
        for _ in 0..n {
            fast = fast.unwrap().next;
        }
        if fast.is_none() {
            return head.unwrap().next;
        }
        while let Some(node) = fast.unwrap().next {
            fast = node.next;
            slow = slow.unwrap().next.as_mut();
        }
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
        head
    }
}
