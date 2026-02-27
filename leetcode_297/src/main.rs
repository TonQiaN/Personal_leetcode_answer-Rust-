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
use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = String::new();
        fn preorder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut String) {
            let Some(node) = root else {
                ans.push_str("#,");
                return;
            };
            let (val, left_node, right_node) = {
                let n = node.borrow();
                (n.val, n.left.clone(), n.right.clone())
            };
            preorder(left_node, ans);
            preorder(right_node, ans);
            ans.push_str(&format!("{},", val));
        }
        preorder(root, &mut ans);
        ans.pop();
        ans
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        };
        let mut it = data.split(',').rev();
        fn build<'a, I>(it: &mut I) -> Option<Rc<RefCell<TreeNode>>>
        where
            I: Iterator<Item = &'a str>,
        {
            let t = it.next()?;
            if t == "#" {
                return None;
            }
            let val: i32 = t.parse().unwrap();
            let right = build(it);
            let left = build(it);
            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
        build(&mut it)
    }
}

// impl Codec {
//     fn new() -> Self {
//         Codec {}
//     }

//     fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
//         let mut ans = String::new();
//         fn preorder(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut String) {
//             let Some(node) = root else {
//                 ans.push_str("#,");
//                 return;
//             };
//             let (val, left_node, right_node) = {
//                 let n = node.borrow();
//                 (n.val, n.left.clone(), n.right.clone())
//             };
//             ans.push_str(&format!("{},", val));
//             preorder(left_node, ans);
//             preorder(right_node, ans);
//         }
//         preorder(root, &mut ans);
//         ans.pop();
//         ans
//     }

//     fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
//         if data.is_empty() {
//             return None;
//         };
//         let mut it = data.split(',');
//         fn build<'a, I>(mut it: &mut I) -> Option<Rc<RefCell<TreeNode>>>
//         where
//             I: Iterator<Item = &'a str>,
//         {
//             let t = it.next()?;
//             if t == "#" {
//                 return None;
//             }
//             let val: i32 = t.parse().unwrap();
//             let left = build(it);
//             let right = build(it);
//             Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
//         }
//         build(&mut it)
//     }
// }


/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

fn main() {
    println!("Hello, world!");
}
