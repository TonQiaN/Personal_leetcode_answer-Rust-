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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        use std::collections::HashMap;
        let mut count = 0;
        let mut pre_sum = HashMap::new();
        pre_sum.insert(0, 1);
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i64,
            pre_sum: &mut HashMap<i64, i32>,
            cur_sum: &mut i64,
            count: &mut i32,
        ) {
            if let Some(node) = root {
                let (val, left, right) = {
                    let n = node.borrow();
                    (n.val as i64, n.left.clone(), n.right.clone())
                };
                *cur_sum += val ;
                let diff = *cur_sum - target_sum;
                if pre_sum.contains_key(&diff) {
                    *count += pre_sum[&diff];
                }
                pre_sum.entry(*cur_sum).and_modify(|c| {*c += 1}).or_insert(1);
                dfs(left, target_sum, pre_sum, cur_sum, count);
                dfs(right, target_sum, pre_sum, cur_sum, count);
                pre_sum.entry(*cur_sum).and_modify(|c| {*c -= 1});
                *cur_sum -= val;
            }
        }
        let mut cur_sum = 0;
        dfs(root, target_sum as i64, &mut pre_sum, &mut cur_sum, &mut count);
        count
    }
}

fn main() {
    println!("Hello, world!");
}
