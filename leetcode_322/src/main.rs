struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX - 1; amount + 1];
        dp[0] = 0;
        for coin in coins {
            let coin = coin as usize;
            for w in coin..=amount {
                dp[w] = dp[w].min(dp[w - coin] + 1);
            }
        }

        if dp[amount] == i32::MAX {
            -1
        } else {
            dp[amount]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
