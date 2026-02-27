struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtracking(left: i32, right: i32, path: &mut String, set: &mut HashSet<String>) {
            if left == 0 && right == 0 {
                set.insert(path.clone());
                return;
            }

            for i in 1..=left {
                for _ in 0..i {
                    path.push('(');
                }
                backtracking(left - i, right, path, set);
                for _ in 0..i {
                    path.pop();
                }
            }

            let balance = right - left;
            for i in 1..=right.min(balance) {
                for _ in 0..i {
                    path.push(')');
                }
                backtracking(left, right - i, path, set);
                for _ in 0..i {
                    path.pop();
                }
            }
        }

        let mut set = HashSet::new();
        let mut path = String::new();
        backtracking(n, n, &mut path, &mut set);
        set.into_iter().collect()
    }
}
// impl Solution {
//     pub fn generate_parenthesis(n: i32) -> Vec<String> {
//         let mut ans = vec![];
//         let mut path = String::new();
//         fn backtracking(left: i32, right: i32, path: &mut String, ans: &mut Vec<String>){
//             if left == 0 && right == 0 {
//                 ans.push(path.clone());
//             }

//             if left > 0 {
//                 path.push('(');
//                 backtracking(left - 1, right, path, ans);
//                 path.pop();
//             }

//             if right > left {
//                 path.push(')');
//                 backtracking(left, right - 1, path, ans);
//                 path.pop();
//             }
//         }
//         backtracking(n, n, &mut path, &mut ans);
//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}

