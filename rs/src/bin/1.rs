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

pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {}

pub fn df(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>, currArr: &mut Vec<i32>) {
    match node {
        Some(node) => {
            let r = RefCell::take(&node).left;
            df(r, res, currArr);
        }
        None => todo!(),
    }
}
