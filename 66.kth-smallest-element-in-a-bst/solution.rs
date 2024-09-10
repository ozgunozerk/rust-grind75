use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {

        let mut k = k;
        let mut stack = vec![];
        let mut curr = root;

        loop {
            while let Some(node) = curr {
                curr = node.borrow_mut().left.take();
                stack.push(node);
            }
            if let Some(node) = stack.pop() {
                k -= 1;
                if k == 0 {
                    return node.borrow().val;
                }
                curr = node.borrow_mut().right.take();
            } else {
                panic!("BST too small");
            }
        }

    }
}
