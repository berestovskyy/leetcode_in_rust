//
// Problem 328. Odd Even Linked List (Medium)
// See [on LeetCode](https://leetcode.com/problems/odd-even-linked-list/).
//
// 0ms (100%)/2.1MB
// Space complexity: O(1)
// Runtime complexity: O(n)
//
// TODO: list tests
//

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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut even_head, mut odd_head) = (ListNode::new(-1), ListNode::new(-1));
        let (mut even_cur, mut odd_cur) = (&mut even_head, &mut odd_head);
        let mut c = 1;
        while let Some(mut node) = head {
            head = node.next.take();
            if c & 1 == 0 {
                even_cur.next = Some(node);
                even_cur = even_cur.next.as_mut().unwrap();
            } else {
                odd_cur.next = Some(node);
                odd_cur = odd_cur.next.as_mut().unwrap();
            }
            c += 1;
        }
        odd_cur.next = even_head.next;
        odd_head.next
    }
}
