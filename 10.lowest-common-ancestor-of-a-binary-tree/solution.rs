// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::cmp::Ordering::{Greater, Less};
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(mut root: TreeLink, p: TreeLink, q: TreeLink) -> TreeLink {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        let mut val;
        while let Some(node) = root {
            val = node.borrow().val;
            match (val.cmp(&q), val.cmp(&p)) {
                (Less, Less) => root = node.borrow_mut().right.take(),
                (Greater, Greater) => root = node.borrow_mut().left.take(),
                _ => return Some(node),
            };
        }
        None
    }
}
