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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn inner_find(
            root: Option<Rc<RefCell<TreeNode>>>,
            ans: &mut Vec<i32>,
            prev: &mut Option<i32>,
            count: &mut i32,
            max_count: &mut i32,
        ) {
            if let Some(node) = root {
                let (val, left, right) = {
                    let node = node.borrow();
                    (node.val, node.left.clone(), node.right.clone())
                };
                inner_find(left, ans, prev, count, max_count);
                if let Some(prev_num) = prev {
                    if *prev_num == val {
                        *count += 1;
                        if *count == *max_count {
                            ans.push(val);
                        }
                        if *count > *max_count {
                            ans.clear();
                            ans.push(val);
                            *max_count = *count;
                        }
                    } else {
                        *count = 1;
                        if *max_count == 1 {
                            ans.push(val);
                        }
                    }
                } else {
                    ans.push(val)
                }
                *prev = Some(val);
                inner_find(right, ans, prev, count, max_count);
            }
        }
        let mut ans = vec![];
        let (mut count, mut max_count) = (1, 1);
        let mut prev = None;
        inner_find(root, &mut ans, &mut prev, &mut count, &mut max_count);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
