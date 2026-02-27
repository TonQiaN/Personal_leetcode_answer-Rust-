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
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let Some(node) = root.clone() else {
            return None;
        };
        let (val, left, right) = {
            let node = node.borrow();
            (node.val, node.left.clone(), node.right.clone())
        };
        match val.cmp(&key) {
            Ordering::Less => {
                let node_right = node.borrow_mut().right.take();
                node.borrow_mut().right = Self::delete_node(node_right, key);
            }
            Ordering::Greater => {
                let node_left = node.borrow_mut().left.take();
                node.borrow_mut().left = Self::delete_node(node_left, key);
            }
            Ordering::Equal => match (left, right) {
                (None, None) => {
                    return None;
                }
                (left, None) => {
                    return left;
                }
                (None, right) => {
                    return right;
                }
                (left, right) => {
                    let mut cur = right.clone();
                    loop {
                        let cur_left = cur.clone().unwrap().borrow().left.clone();
                        cur = if cur_left.is_some() {
                            cur_left
                        } else {
                            break;
                        };
                    }
                    let cur_node = cur.unwrap();
                    let mut cur_borrow = cur_node.borrow_mut();
                    cur_borrow.left = left;
                    return right;
                }
            },
        }
        root
    }
}

fn main() {
    println!("Hello, world!");
}
