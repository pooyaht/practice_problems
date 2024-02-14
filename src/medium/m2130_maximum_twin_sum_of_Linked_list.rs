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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut slow = &head;
        let mut fast = &head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }

        let mut prev: Option<Box<ListNode>> = None;
        let mut curr = slow.clone();
        while let Some(mut node) = curr {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            curr = next;
        }

        let mut max_sum = 0;
        let mut head = head;
        while prev.is_some() {
            max_sum = max_sum.max(head.as_ref().unwrap().val + prev.as_ref().unwrap().val);
            head = head.unwrap().next;
            prev = prev.unwrap().next;
        }
        max_sum
    }
}
