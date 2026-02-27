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
use std::result;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // let mut ans = vec![];
        // fn inorder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        //   if let Some(node) = root {
        //     let node = node.borrow();
        //     inorder(node.left.clone(), ans);
        //     ans.push(node.val);
        //     inorder(node.right.clone(), ans);
        //   }
        // }
        // inorder(root, &mut ans);
        // ans
        let mut ans = vec![];
        let mut my_stack = vec![];
        let mut curr = root.clone();
        while !my_stack.is_empty() || !curr.is_none(){
            if let Some(node) = curr {
                my_stack.push(node.clone());
                curr = node.borrow().left.clone();
            } else {
                let node = my_stack.pop().unwrap();
                let node_borrow = node.borrow();
                ans.push(node_borrow.val);
                curr = node_borrow.right.clone();
              }
        }
        ans
    }
}

fn main() {}
