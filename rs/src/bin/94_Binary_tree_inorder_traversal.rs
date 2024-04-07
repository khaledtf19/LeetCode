use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {
    unimplemented!();
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    match root {
        Some(node) => recurse(&mut res, &node),
        None => return vec![],
    }
    return res;
}

pub fn recurse(res: &mut Vec<i32>, root: &Rc<RefCell<TreeNode>>) {
    match &root.as_ref().borrow().left {
        Some(node) => recurse(res, node),
        None => {}
    }
    res.push(root.as_ref().borrow().val.clone());
    println!("{:?}", res);
    match &root.as_ref().borrow().right {
        Some(node) => recurse(res, node),
        None => {}
    }
}
