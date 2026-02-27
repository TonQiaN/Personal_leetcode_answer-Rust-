use std::path;

struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut st = vec![];
        st.push((0, vec![0]));
        while let Some((u, path)) = st.pop() {
            if u == graph.len() - 1 {
                ans.push(path.clone());
                continue;
            }
            for &v in &graph[u] {
                let mut next_path = path.clone();
                next_path.push(v);
                st.push((v as usize, next_path));
            }
        }
        ans
    }
}

// impl Solution {
//     pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         let mut ans = vec![];
//         let mut path = vec![0];
//         fn dfs(
//             graph: &Vec<Vec<i32>>,
//             path: &mut Vec<i32>,
//             ans: &mut Vec<Vec<i32>>,
//             u: usize,
//         ) {
//             if u == graph.len() - 1 {
//                 ans.push(path.clone());
//                 return;
//             }
//             for &v in &graph[u] {
//                 path.push(v);
//                 dfs(graph, path, ans, v as usize);
//                 path.pop();
//             }
//         }
//         dfs(&graph, &mut path, &mut ans, 0);
//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
