struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        cost.into_iter().chain(std::iter::once(0)).fold((0, 0), |(pp, p), c| (p, c + pp.min(p))).1
    }
    // pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    //     let n = cost.len();
    //     let mut steps = vec![0; n + 1];
    //     for i in 2..n + 1 {
    //         steps[i] = i32::min(steps[i - 1] + cost[i - 1], steps[i - 2] + cost[i - 2]);
    //     }
    //     steps[n - 1]
    // }
}

fn main() {
    println!("Hello, world!");
}
