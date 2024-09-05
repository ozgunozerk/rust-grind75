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
use std::rc::Rc;
use std::cell::RefCell;

type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn lowest_common_ancestor(node: OptNode, p: OptNode, q: OptNode) -> OptNode {
        if node.is_none() {
            return None;
        }

        if node == p || node == q {
            return node;
        }

        let node = node.expect("is_none check is already done");
        let left = Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());

        // if we find a p and q on the different sides of the root,
        // then root is the LCA
        // if both of p and q are on the same side, then the first encounter
        // (either p or q) is the LCA
        // we cannot find p and q, there is no LCA
        match (left, right) {
            (Some(_), Some(_)) => Some(node),
            (Some(tree), None) => Some(tree),
            (None, Some(tree)) => Some(tree),
            (None, None) => None,
        }
    }
}
