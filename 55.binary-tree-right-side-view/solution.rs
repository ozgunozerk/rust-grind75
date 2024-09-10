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
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // Check if the root is None
        if root.is_none() {
            return vec![];
        }
        // Initialize an empty result array to store the right side view of the binary tree.
        let mut result = vec![];

        // Create a queue to perform the level-order traversal.
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        /*
        While the queue is not empty, do the following:
        - Get the size of the current level by accessing the queue's length.
        - Traverse through the nodes at the current level (based on the size obtained).
        - Dequeue the node from the front of the queue.
        - Check if the current node has a left child. If it does, enqueue the left child.
        - Check if the current node has a right child. If it does, enqueue the right child.
        - Add the last node's value to the result array.
        */

        while !queue.is_empty() {
            let queue_size = queue.len();

            // we don't want to check in each `node` in for loop iteration to
            // see whether they are the last node. So we will move `node` variable
            // outside of the for loop with a dummy value initialization,
            // and use it after the loop to get the last node in the current level
            let mut node = Rc::new(RefCell::new(TreeNode::new(0)));

            for index in 0..queue_size {
                node = queue.pop_front().unwrap();
                let left = &node.borrow().left;
                let right = &node.borrow().right;

                if left.is_some() {
                    queue.push_back(Rc::clone(left.as_ref().unwrap()));
                }

                if right.is_some() {
                    queue.push_back(Rc::clone(right.as_ref().unwrap()));
                }
            }
            result.push(node.borrow().val);
        }
        result
    }
}
