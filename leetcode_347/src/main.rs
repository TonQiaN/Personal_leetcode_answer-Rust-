
struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::HashMap;
        use std::collections::BinaryHeap;
        let k = k as usize;
        let mut frequency_map = HashMap::new();

        for &num in nums.iter() {
            *frequency_map.entry(num).or_insert(0) += 1;
        }

        let mut small_heap = BinaryHeap::new();
        for (key, v) in frequency_map {
            small_heap.push(Reverse((v, key)));
            if small_heap.len() > k {
                small_heap.pop();
            }
        }

        small_heap.into_iter().map(|Reverse(kv_pair)| kv_pair.1).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
