struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid.first().map_or(0, |r0| r0.len()));
        let mut dp = vec![i32::MAX; m];
        dp[0] = 0;
        for i in 0..n {
            for j in 0..m {
                let left = if j > 0 {dp[j - 1]} else {i32::MAX};
                dp[j] = dp[j].min(left) + grid[i][j];
            }
        }
        dp[m - 1]
    }
}
// impl Solution {
//     pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
//         let (n, m) = (grid.len(), grid.first().map_or(0, |r0| r0.len()));
//         let mut dp = vec![vec![i32::MAX; m + 1]; n + 1];
//         dp[0][1] = 0;
//         for i in 1..n + 1 {
//             for j in 1..m + 1 {
//                 dp[i][j] = i32::min(dp[i][j - 1], dp[i - 1][j]) + grid[i - 1][j - 1];
//             }
//         }
//         dp[n][m]
//     }
// }

fn main() {
    println!("Hello, world!");
}
