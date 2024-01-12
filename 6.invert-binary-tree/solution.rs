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
use std::rc::Rc;

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn invert_tree(root: TreeLink) -> TreeLink {
        let mut stack: Vec<TreeLink> = vec![root.clone()];
        while let Some(maybe_node) = stack.pop() {
            if let Some(node) = maybe_node {
                let TreeNode { left, right, .. } = &mut *node.borrow_mut();
                std::mem::swap(right, left);
                stack.push(left.clone());
                stack.push(right.clone());
            }
        }
        root
    }
}
