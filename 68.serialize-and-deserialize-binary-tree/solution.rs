use std::rc::Rc;
use std::cell::RefCell;
struct Codec;
use std::fmt::Write as _;


impl Codec {
    fn new() -> Self {
        Self {}
    }


    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut out = String::new();
        Self::serialize_node(root.as_ref().map(|v| v.borrow()).as_deref(), &mut out);
        out
    }

    fn serialize_node(node: Option<&TreeNode>, out: &mut String) {
        if let Some(node) = node {
            writeln!(out, "{}", node.val);
            Self::serialize_node(node.left.as_ref().map(|v| v.borrow()).as_deref(), out);
            Self::serialize_node(node.right.as_ref().map(|v| v.borrow()).as_deref(), out);
        } else {
            writeln!(out, "x").unwrap();
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut itr = data.lines().map(|v| v.parse().ok());
        Self::deserialize_node(&mut itr)
    }

    fn deserialize_node(data: &mut impl Iterator<Item = Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let val = data.next()??;
        let left = Self::deserialize_node(data);
        let right = Self::deserialize_node(data);
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

}
