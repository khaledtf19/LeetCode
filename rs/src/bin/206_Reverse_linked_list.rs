fn main() {
    unimplemented!();
}

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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr_head: Option<Box<ListNode>> = None;
    let mut curr = &head;
    while let Some(node) = curr {
        let mut next = ListNode::new(node.val);
        next.next = curr_head;
        curr_head = Some(Box::new(next));
        curr = &node.next;
    }
    curr_head
}
