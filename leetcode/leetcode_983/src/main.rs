use std::i32;

struct Solution {}

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        // let mut dp_arr = [i32::MAX; 366];
        // fn dp(days: &[i32], costs: &Vec<i32>, dp_arr: &mut [i32], i: usize) -> i32 {
        //     let mut best = i32::MAX;
        //     let plan = [1, 7, 30];
        //     if i >= days.len() {
        //         return 0;
        //     }
        //     if dp_arr[i] != i32::MAX {
        //         return dp_arr[i];
        //     }
        //     for k in 0..3 {
        //         let mut j = i;
        //         while j < days.len() && days[i] + plan[k] > days[j] {
        //             j += 1;
        //         }
        //         best = best.min(costs[k] + dp(days, costs, dp_arr, j));
        //     }
        //     dp_arr[i] = dp_arr[i].min(best);
        //     best
        // }
        // dp(&days, &costs, &mut dp_arr, 0)

        let mut dp = [i32::MAX; 366];
        let n = days.len();
        let plan = [1, 7, 30];
        dp[n] = 0;
        for i in (0..n).rev() {
            for k in 0..3 {
                let mut j = i;
                while j < n && days[j] < days[i] + plan[k] {
                    j += 1;
                }
                dp[i] = dp[i].min(costs[k] + dp[j]);
            }
        }
        dp[0]
    }
}

fn main() {
    println!("Hello, world!");
}
