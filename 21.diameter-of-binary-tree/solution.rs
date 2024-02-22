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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(root).1
    }
}

fn dfs(maybe_node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    match maybe_node {
        None => (0, 0),
        Some(node) => {
            let (left_depth, left_dia) = dfs(node.borrow().left.clone());
            let (right_depth, right_dia) = dfs(node.borrow().right.clone());
            (
                i32::max(left_depth, right_depth) + 1,
                i32::max(left_dia, i32::max(right_dia, left_depth + right_depth)),
            )
        }
    }
}

/*
# Explanation

Naive approach: calculate maximum left depth and maximum right depth for a node, and then add them up.

However, this approach fails when there is a longer way within left or right node. For example, the root node might have only a single node in left, but a very complex tree in right node.

In this case, taking the maximum depth of left node will return us 1, and taking the maximum depth of right will, say, 5.

But, what if right node of root itself has 4 nodes in the left branch, and 4 nodes in the right branch?

To cover this case, we also calculate the maximum diameter for each node, and compare our naive result (addition of left and right depth) to the maximum of right and left diameters
*/
