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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut a_ptr = &mut list1;
        let b_ptr = &mut list2;

        loop {
            if a_ptr.is_none() {
                std::mem::swap(a_ptr, b_ptr);
                return list1;
            }
            if b_ptr.is_none() {
                return list1;
            }
            if a_ptr.as_ref().unwrap().val > b_ptr.as_ref().unwrap().val {
                std::mem::swap(a_ptr, b_ptr);
                a_ptr = &mut a_ptr.as_mut().unwrap().next;
            } else {
                a_ptr = &mut a_ptr.as_mut().unwrap().next;
            }
        }
    }
}
