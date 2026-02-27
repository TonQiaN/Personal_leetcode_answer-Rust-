struct Solution {}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::VecDeque;

        let num_courses = num_courses as usize;
        let mut graph = vec![vec![]; num_courses];
        let mut indegree = vec![0; num_courses];
        let mut queue = VecDeque::new();
        let mut ans = vec![];

        for prerequisit in prerequisites {
            let (u, v) = (prerequisit[1] as usize, prerequisit[0] as usize);
            graph[u].push(v);
            indegree[v] += 1;
        }
        for i in 0..num_courses {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }

        while let Some(course) = queue.pop_front() {
            ans.push(course as i32);
            for &v in &graph[course] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if ans.len() == num_courses {
            ans
        } else {
            vec![]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
