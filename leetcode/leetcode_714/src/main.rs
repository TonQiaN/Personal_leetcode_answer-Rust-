struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        let mut buy = -prices[0];
        let mut sell = 0;
        for i in 1..n {
            buy = buy.max(sell - prices[i]);
            sell = sell.max(buy + prices[i] - fee);
        }
        sell
    }
}
// impl Solution {
//     pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
//         let n = prices.len();
//         let mut dp = vec![vec![i32::MIN, 0]; n];
//         dp[0][0] = -prices[0];
//         for i in 1..n {
//             dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] - prices[i]);
//             dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + prices[i] - fee);
//         }
//         dp[n - 1][1]
//     }
// }

fn main() {
    println!("Hello, world!");
}
