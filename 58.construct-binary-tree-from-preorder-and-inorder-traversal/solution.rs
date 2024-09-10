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
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = TreeNode::new(preorder[0]);
        // Split the inorder vec
        let i = inorder.iter().position(|&x| x == preorder[0])?;
        let inorder_left = inorder[0..i].to_vec();
        let inorder_right = inorder[i+1..].to_vec();

        // Split the preorder vec
        let preorder_left = preorder[1..1+inorder_left.len()].to_vec();
        let preorder_right = preorder[1+inorder_left.len()..].to_vec();

        if inorder_left.len() > 0 {
            root.left = Self::build_tree(preorder_left, inorder_left);
        }
        if inorder_right.len() > 0 {
            root.right = Self::build_tree(preorder_right, inorder_right);
        }

        Some(Rc::new(RefCell::new(root)))
    }
}
