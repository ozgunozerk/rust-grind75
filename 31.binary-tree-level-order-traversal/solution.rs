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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(1000);
        let mut roots = VecDeque::with_capacity(2000);

        roots.push_back((0, root));

        while let Some((lvl, maybe_node)) = roots.pop_front() {
            if let Some(node) = maybe_node {
                let mut node_ref = node.borrow_mut();
                if lvl >= res.len() {
                    res.resize(lvl + 1, vec![])
                }
                res[lvl].push(node_ref.val);
                roots.push_back((lvl + 1, node_ref.left.take()));
                roots.push_back((lvl + 1, node_ref.right.take()));
            }
        }
        res
    }
}
