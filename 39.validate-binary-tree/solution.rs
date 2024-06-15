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
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_sub_tree_valid(root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
    }

    pub fn is_sub_tree_valid(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(node) => {
                let node_val = node.borrow().val as i64;
                if node_val <= min || node_val >= max {
                    return false;
                }

                Self::is_sub_tree_valid(node.borrow_mut().left.take(), min, node_val)
                    && Self::is_sub_tree_valid(node.borrow_mut().right.take(), node_val, max)
            }
        }
    }
}
