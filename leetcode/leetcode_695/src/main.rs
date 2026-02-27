struct Solution {}

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let mut ans = 0;
        let mut grid = grid;
        let mut queue = VecDeque::new();
        let (n, m) = (grid[0].len(), grid.len());
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    queue.push_back((i, j));
                    let mut area = 1;
                    while !queue.is_empty() {
                        let (cur_i, cur_j) = queue.pop_front().unwrap();
                        if cur_i + 1 < m && grid[cur_i + 1][cur_j] == 1 {
                            area += 1;
                            grid[cur_i + 1][cur_j] = 0;
                            queue.push_back((cur_i + 1, cur_j));
                        }
                        if cur_j + 1 < n && grid[cur_i][cur_j + 1] == 1 {
                            area += 1;
                            grid[cur_i][cur_j + 1] = 0;
                            queue.push_back((cur_i, cur_j + 1));
                        }
                        if cur_i > 0 && grid[cur_i - 1][cur_j] == 1 {
                            area += 1;
                            grid[cur_i - 1][cur_j] = 0;
                            queue.push_back((cur_i - 1, cur_j));
                        }
                        if cur_j > 0 && grid[cur_i][cur_j - 1] == 1 {
                            area += 1;
                            grid[cur_i][cur_j - 1] = 0;
                            queue.push_back((cur_i, cur_j - 1));
                        }
                    }
                    ans = ans.max(area);
                }
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
