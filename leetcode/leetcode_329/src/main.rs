struct Solution {}

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix.first().map_or(0, |r0| r0.len());
        let mut longgest = 1;
        let mut dp = vec![vec![0; m]; n];

        fn find_path(
            matrix: &Vec<Vec<i32>>,
            i: usize,
            j: usize,
            dp: &mut Vec<Vec<usize>>,
        ) -> usize {
            if dp[i][j] != 0 {
                return dp[i][j];
            } 
            dp[i][j] = 1;
            let n = matrix.len();
            let m = matrix.first().map_or(0, |r0| r0.len());
            let neighbours = [
                (i + 1, j),
                (i, j + 1),
                (i.wrapping_sub(1), j),
                (i, j.wrapping_sub(1)),
            ];
            for (i1, j1) in neighbours {
                if i1 < n && j1 < m && matrix[i1][j1] > matrix[i][j] {
                    dp[i][j] = dp[i][j].max(find_path(matrix, i1, j1, dp) + 1);
                }
            }
            dp[i][j]
        }
        for i in 0..n {
            for j in 0..m {
                longgest = longgest.max(find_path(&matrix, i, j, &mut dp));
            }
        }
        longgest as i32
    }
}

fn main() {
    println!("Hello, world!");
}
