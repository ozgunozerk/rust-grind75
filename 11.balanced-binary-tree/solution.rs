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

type Node = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_balanced(root: Option<Node>) -> bool {
        fn tree_processor(root: Option<Node>) -> Option<isize> {
            match root {
                None => Some(0),
                Some(node) => {
                    let left = tree_processor(node.borrow().left.clone())?;
                    let right = tree_processor(node.borrow().right.clone())?;
                    ((left - right).abs() <= 1).then(|| left.max(right) + 1)
                }
            }
        }
        tree_processor(root).map(|_| true).unwrap_or_default()
    }
}
