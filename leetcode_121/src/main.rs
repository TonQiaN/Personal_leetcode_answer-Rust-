struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![i32::MIN, i32::MIN]; n];
        dp[0][0] = -prices[0];
        for i in 1..n {
            dp[i][0] = i32::max(dp[i - 1][0], -prices[i]);
            dp[i][1] = i32::max(dp[i - 1][1], dp[i - 1][0] + prices[i]);
        }
        if dp[n - 1][1] < 0 {
            0
        } else {
            dp[n - 1][1]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
