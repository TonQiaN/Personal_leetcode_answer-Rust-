struct Solution {}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for str in strs {
            let num_0 = str.chars().filter(|&c| c == '0').count();
            let num_1 = str.len() - num_0;
            for i in (num_0..=m).rev() {
                for j in (num_1..=n).rev() {
                    dp[i][j] = dp[i][j].max(dp[i - num_0][j - num_1] + 1);
                }
            }
        }

        dp[m][n]
    }
}

fn main() {
    println!("Hello, world!");
}
