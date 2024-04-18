use std::cell::RefCell;
use std::rc::Rc;
use std::{i32, usize};

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
    let res = sorted_array_to_bst(vec![-20, -16, -15, -10, -3, 0, 5, 9, 10, 15, 20]);
    dbg!(res);
}
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    recurse(&nums, 0, nums.len() as i32 - 1)
}

pub fn recurse(nums: &Vec<i32>, start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let mid = start + (end - start) / 2;
    if nums.get(mid as usize).is_none() || start < 0 || start > end {
        return None;
    }
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: nums[mid as usize],
        left: recurse(nums, start, mid - 1),
        right: recurse(nums, mid + 1, end),
    })));

    return root;
}

// [-20,-16,-15,-10, -3,0,5,9,10,15,20];
//
