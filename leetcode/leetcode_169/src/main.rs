struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut frequency_map = HashMap::new();
        let majority_time = nums.len() /2;
        for num in nums {
            *frequency_map.entry(num).or_insert(0) += 1;
        }
        frequency_map.into_iter()
        .find(|&(_, v)| v > majority_time)
        .map(|(k, _)| k)
        .unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
