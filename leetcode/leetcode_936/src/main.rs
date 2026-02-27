struct Solution {}

impl Solution {
    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        use std::collections::VecDeque;
        let (n, m) = (target.len(), stamp.len());
        let (stamp_bytes, target_bytes) = (stamp.as_bytes(), target.as_bytes());
        let mut path = vec![];
        let mut queue = VecDeque::new();
        let mut indegree = vec![m; n - m + 1];
        let mut graph = vec![vec![]; n];
        let mut visited = vec![false; n];
        
        for i in 0..(n - m + 1) {
            for j in 0..m {
                if target_bytes[i + j] == stamp_bytes[j] {
                    indegree[i] -= 1;
                    if indegree[i] == 0 {
                        path.push(i as i32);
                        queue.push_back(i);
                    }
                } else {
                    graph[i + j].push(i);
                }
            }
        }

        while let Some(i) = queue.pop_front() {
            for j in 0..m {
                if !visited[i + j] {
                    visited[i + j] = true;
                    for &next in &graph[i + j] {
                        indegree[next] -= 1;
                        if indegree[next] == 0 {
                            path.push(next as i32);
                            queue.push_back(next);
                        }
                    }
                }
            }
        }

        if path.len() != n - m + 1 {
            return vec![];
        }

        path.reverse();
        path
    }
}

fn main() {
    println!("Hello, world!");
}
