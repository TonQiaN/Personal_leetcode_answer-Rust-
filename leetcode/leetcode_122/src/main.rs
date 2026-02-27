use std::i32;

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut buy = i32::MIN;
        let mut sell = 0;
        for i in 0..n {
            buy = buy.max(sell - prices[i]);
            sell = sell.max(buy + prices[i]);
        }
        sell
    }
}
// impl Solution {
//     pub fn max_profit(prices: Vec<i32>) -> i32 {
//         let n = prices.len();
//         let mut dp = vec![vec![i32::MIN, 0]; n];
//         dp[0][0] = -prices[0];
//         for i in 1..n {
//             dp[i][0] = i32::max(dp[i - 1][0], dp[i - 1][1] - prices[i]);
//             dp[i][1] = i32::max(dp[i - 1][1], dp[i - 1][0] + prices[i]);
//         }

//         if dp[n - 1][1] < 0 { 0 } else { dp[n - 1][1] }
//     }
// }

// impl Solution {
//     pub fn max_profit(prices: Vec<i32>) -> i32 {
//         prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum()
//     }
// }

fn main() {
    println!("Hello, world!");
}
