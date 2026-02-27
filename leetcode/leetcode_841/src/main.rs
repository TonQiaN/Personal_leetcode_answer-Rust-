struct Solution {}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut visited = vec![false; rooms.len()];
        visited[0] = true;
        queue.extend(rooms[0].iter().copied());
        while let Some(ni) = queue.pop_front() {
            let ni = ni as usize;
            if visited[ni] {
                continue;
            }

            visited[ni] = true;
            queue.extend(rooms[ni].iter().copied());
        }
        visited.iter().all(|&flag| flag)
    }
}

fn main() {
    println!("Hello, world!");
}
