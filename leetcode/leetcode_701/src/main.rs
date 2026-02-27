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
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        // if let Some(node) = root.clone() {
        //     let (node_val, left, right) = {
        //         let nb = node.borrow();
        //         (nb.val, nb.left.clone(), nb.right.clone())
        //     };

        //     match node_val.cmp(&val) {
        //         Ordering::Less => {
        //             if let Some(r) = right {
        //                 Self::insert_into_bst(Some(r), val);
        //             } else {
        //                 node.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        //             }
        //         }
        //         Ordering::Greater => {
        //             if let Some(l) = left {
        //                 Self::insert_into_bst(Some(l), val);
        //             } else {
        //                 node.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        //             }
        //         }
        //         Ordering::Equal => {}
        //     }
        //     root
        // } else {
        //     Some(Rc::new(RefCell::new(TreeNode::new(val))))
        // }
        
        if let Some(node) = root {
            {let mut nb = node.borrow_mut();
            match nb.val.cmp(&val) {
                Ordering::Less => {
                    let right = nb.right.take();
                    nb.right = Self::insert_into_bst(right, val);
                },
                Ordering::Greater => {
                    let left = nb.left.take();
                    nb.left = Self::insert_into_bst(left, val);
                },
                Ordering::Equal => {},
            }}
            Some(node)
        } else {
            Some(Rc::new(RefCell::new(TreeNode::new(val))))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
