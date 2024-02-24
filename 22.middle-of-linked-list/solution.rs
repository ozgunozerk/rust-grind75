// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head;
        while let Some(ListNode {
            next: Some(node), ..
        }) = fast.take().map(|x| *x)
        {
            slow = slow.unwrap().next;
            fast = node.next;
        }
        slow
    }
}
