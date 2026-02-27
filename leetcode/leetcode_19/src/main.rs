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

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // let mut dummy = Box::new(ListNode { val: 0, next: head.clone() });

        // let mut len = 0;
        // let mut p = head.as_ref();
        // while let Some(node) = p {
        //     len += 1;
        //     p = node.next.as_ref();
        // }
        // let mut prev = dummy.as_mut();
        // for _ in 0..(len - n as usize) {
        //     prev = prev.next.as_mut().unwrap();
        // }
        // let delete = prev.next.take().unwrap();
        // prev.next = delete.next;
        // dummy.next

        let mut dummy = Box::new(ListNode {
            val: 0,
            next: head.clone(),
        });

        let mut fast = head.as_ref();
        for _ in 0..n - 1 {
            fast = fast.unwrap().next.as_ref();
        }
        let mut slow = dummy.as_mut();
        while fast.is_some() {
            fast = fast.unwrap().next.as_ref();
            slow = slow.next.as_mut().unwrap();
        }
        let delete = slow.next.take().unwrap();
        slow.next = delete.next;
        dummy.next
    }
}

fn main() {
    println!("Hello, world!");
}
