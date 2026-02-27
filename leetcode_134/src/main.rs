struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut cur_sum = 0;
        let mut total_sum = 0;
        // if gas.iter().sum::<i32>() < cost.iter().sum() {
        //     return -1;
        // }
        for i in 0..gas.len() {
            cur_sum += gas[i] - cost[i];
            total_sum += gas[i] - cost[i];
            if cur_sum < 0 {
                cur_sum = 0;
                start = i + 1;
            }
        }
        if total_sum < 0 {
            return -1;
        }
        start as i32
    }
}

fn main() {
    println!("Hello, world!");
}
