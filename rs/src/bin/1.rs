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
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    pseudo_palindromic_paths(root);
}

pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut curr_arr = vec![0; 9];
    let count = df(&root, &mut curr_arr, 0);
    println!("1:{:?}, 2:{:?}", curr_arr, count);
    count
}

pub fn df(node: &Option<Rc<RefCell<TreeNode>>>, curr_arr: &mut Vec<i32>, mut odd: i32) -> i32 {
    match node {
        Some(node) => {
            let node = node.borrow();
            curr_arr[node.val as usize] += 1;

            let mut odds_count = odd;
            odds_count += (curr_arr[node.val as usize] % 2 != 0) as i32;

            odds_count -= (curr_arr[node.val as usize] % 2 == 0) as i32;

            let mut count = 0;
            let left = &node.left;
            count += df(&left, curr_arr, odd);
            let right = &node.right;
            count += df(&right, curr_arr, odd);
            if left == &None && right == &None {
                count = (odds_count <= 1) as i32;
            }
            curr_arr[node.val as usize] -= 1;
            count
        }
        None => {
            return 0;
        }
    }
}
