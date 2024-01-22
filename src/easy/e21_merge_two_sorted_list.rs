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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l), None) => Some(l),
            (None, Some(r)) => Some(r),
            (None, None) => None,
            (Some(l), Some(r)) => {
                if l.val <= r.val {
                    Some(Box::new(ListNode {
                        next: Self::merge_two_lists(l.next, Some(r)),
                        val: l.val,
                    }))
                } else {
                    Some(Box::new(ListNode {
                        next: Self::merge_two_lists(Some(l), r.next),
                        val: r.val,
                    }))
                }
            }
        }
    }
}
