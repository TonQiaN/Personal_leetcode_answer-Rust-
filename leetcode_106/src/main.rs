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

use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if postorder.is_empty() {
                return None;
            } else {
                let &root_value = postorder.last().unwrap();
                let mut root = TreeNode::new(root_value);
                if postorder.len() == 1 {
                    return Some(Rc::new(RefCell::new(root)));
                } else {
                    let idx = inorder.iter().position(|&x| x == root_value).unwrap();
                    let (left_inorder, right_inorder) = {
                        let (left_inorder, right_inorder_with_first) = inorder.split_at(idx);
                        let (_, right_inorder) = right_inorder_with_first.split_first().unwrap();
                        (left_inorder, right_inorder)
                    };
                    let left_len = left_inorder.len();
                    let (left_postorder, right_postorder) = {
                        let (left_postorder, right_postorder_with_last) = postorder.split_at(left_len);
                        let (_, right_postorder) = right_postorder_with_last.split_last().unwrap();
                        (left_postorder, right_postorder)
                    };
                    root.left = build(left_inorder, left_postorder);
                    root.right = build(right_inorder, right_postorder);
                    Some(Rc::new(RefCell::new(root)))
                }
            }
        }
        build(&inorder, &postorder)
    }
}

fn main() {
    println!("Hello, world!");
}
