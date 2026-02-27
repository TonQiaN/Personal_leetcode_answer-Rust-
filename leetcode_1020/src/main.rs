struct Solution {}

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let mut ans = 0;
        let (n, m) = (grid[0].len(), grid.len());
        let mut grid = grid;
        let mut queue = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let mut flag = false;
                    let mut area = 1;
                    grid[i][j] = 0;
                    queue.push_back((i, j));
                    while !queue.is_empty() {
                        let (x, y) = queue.pop_front().unwrap();
                        if x == m - 1 || x == 0 || y == n - 1 || y == 0 {
                            flag = true;
                        }
                        if x + 1 < m && grid[x + 1][y] == 1 {
                            area += 1;
                            grid[x + 1][y] = 0;
                            queue.push_back((x + 1, y));
                        }
                        if x > 0 && grid[x - 1][y] == 1 {
                            area += 1;
                            grid[x - 1][y] = 0;
                            queue.push_back((x - 1, y));
                        }
                        if y + 1 < n && grid[x][y + 1] == 1 {
                            area += 1;
                            grid[x][y + 1] = 0;
                            queue.push_back((x, y + 1));
                        }
                        if y > 0 && grid[x][y - 1] == 1 {
                            area += 1;
                            grid[x][y - 1] = 0;
                            queue.push_back((x, y - 1));
                        }
                    }
                    if !flag {
                        ans += area;
                    }
                }
            }
        }

        ans
    }
}
// impl Solution {
//     pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
//         let mut ans = 0;
//         let (n, m) = (grid[0].len(), grid.len());
//         let mut grid = grid;

//         fn dfs(grid: &mut Vec<Vec<i32>>, flag: &mut bool, i: usize, j: usize, area: &mut i32) {
//             grid[i][j] = 0;
//             *area += 1;
//             let (n, m) = (grid[0].len(), grid.len());

//             if i == m - 1 || i == 0 || j == n - 1 || j == 0 {
//                 *flag = true;
//             }

//             if i + 1 < m && grid[i + 1][j] == 1 {
//                 dfs(grid, flag, i + 1, j, area);
//             }
//             if i > 0 && grid[i - 1][j] == 1 {
//                 dfs(grid, flag, i - 1, j, area);
//             }
//             if j + 1 < n && grid[i][j + 1] == 1 {
//                 dfs(grid, flag, i, j + 1, area);
//             }
//             if j > 0 && grid[i][j - 1] == 1 {
//                 dfs(grid, flag, i, j - 1, area);
//             }
//         }

//         for i in 0..m {
//             for j in 0..n {
//                 if grid[i][j] == 1 {
//                     let mut flag = false;
//                     let mut area = 0;
//                     dfs(&mut grid, &mut flag, i, j, &mut area);
//                     if !flag {
//                         ans += area;
//                     }
//                 }
//             }
//         }

//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
