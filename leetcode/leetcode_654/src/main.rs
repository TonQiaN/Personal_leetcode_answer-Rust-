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

use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn construct(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            if nums.is_empty() {
                None
            } else {
                let (max_index, &max_value) = nums
                    .iter()
                    .enumerate()
                    .max_by_key(|&(i, &num)| num)
                    .unwrap();
                let mut root = TreeNode::new(max_value);
                let (left_nums, right_nums_with_max) = nums.split_at(max_index);
                let (_, right_nums) = right_nums_with_max.split_first().unwrap();
                root.left = construct(left_nums);
                root.right = construct(right_nums);
                Some(Rc::new(RefCell::new(root)))
            }
        }
        construct(&nums)
    }
}

fn main() {
    println!("Hello, world!");
}
