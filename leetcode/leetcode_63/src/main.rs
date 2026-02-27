struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                } else {
                    match (i, j) {
                        (0, 0) => dp[0][0] = if obstacle_grid[0][0] == 1 { 0 } else { 1 },
                        (0, j) => dp[0][j] = if dp[0][j - 1] == 0 { 0 } else { 1 },
                        (i, 0) => dp[i][0] = if dp[i - 1][0] == 0 { 0 } else { 1 },
                        (i, j) => dp[i][j] = dp[i - 1][j] + dp[i][j - 1],
                    }
                }
            }
        }
        dp[m - 1][n - 1]
    }
}

fn main() {
    println!("Hello, world!");
}
