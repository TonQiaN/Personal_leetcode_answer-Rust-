struct Solution {}

impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let total_weight = stones.iter().sum::<i32>();
        let target = (total_weight / 2) as usize;
        let mut dp = vec![0; target as usize + 1];

        for stone in stones {
            let stone = stone as usize;
            if stone > target {
                continue;
            }
            for w in (stone..=target).rev() {
                dp[w] = dp[w].max(dp[w - stone] + stone);
            }
        }

        total_weight - 2 * dp[target] as i32
    }
}

fn main() {
    println!("Hello, world!");
}
