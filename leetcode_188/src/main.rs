use std::i32;

struct Solution {}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut buy: Vec<i32> = vec![i32::MIN; k + 1];
        let mut sell: Vec<i32> = vec![0; k + 1];
        for i in 0..prices.len() {
            for j in 1..=k {
                buy[j] = i32::max(buy[j], sell[j - 1] - prices[i]);
                sell[j] = i32::max(sell[j], buy[j] + prices[i]);
            }
        }
        sell[k]
    }
}

// impl Solution {
//     pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
//         let n = prices.len();
//         let k = k as usize;
//         let mut inner_vec = vec![0; 2 * k];
//         for t in 0..k {
//             inner_vec[2 * t] = i32::MIN;
//         }
//         let mut dp = vec![inner_vec; n];
//         for t in 0..k {
//             dp[0][2 * t] = -prices[0];
//         }

//         for i in 1..n {
//             dp[i][0] = i32::max(dp[i - 1][0], -prices[i]);
//             dp[i][1] = i32::max(dp[i - 1][1], dp[i - 1][0] + prices[i]);
//             for t in 1..k {
//                 dp[i][2 * t] = i32::max(dp[i - 1][2 * t], dp[i - 1][2 * t - 1] - prices[i]);
//                 dp[i][2 * t + 1] = i32::max(dp[i - 1][2 * t + 1], dp[i - 1][2 * t] + prices[i]);
//             }
//         }
//         dp[n - 1][2 * k as usize - 1]
//     }
// }

fn main() {
    println!("Hello, world!");
}
