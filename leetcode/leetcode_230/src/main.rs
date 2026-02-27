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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, ans: &mut i32) {
            if let Some(node) = root {
                let (left, right) = {
                    let n = node.borrow();
                    (n.left.clone(), n.right.clone())
                };
                dfs(left, k, ans);
                *k -= 1;
                if *k == 0 {
                    *ans = node.borrow().val;
                }
                dfs(right, k, ans);
            }
        }
        let mut ans = 0;
        let mut k = k;
        dfs(root, &mut k, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
