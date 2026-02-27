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

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inner_convert(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut i32) {
            if let Some(node) = root {
                let (val, left, right) = {
                    let node = node.borrow();
                    (node.val, node.left.clone(), node.right.clone())
                };
                inner_convert(&right, prev);
                *prev += val;
                node.borrow_mut().val = *prev;
                inner_convert(&left, prev );
            }
        }

        let mut prev = 0;
        inner_convert(&root, &mut prev);
        root
    }
}

fn main() {
    println!("Hello, world!");
}
