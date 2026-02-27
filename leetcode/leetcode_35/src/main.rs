struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => left = mid + 1,
                Ordering::Equal => return (mid + 1) as i32,
                Ordering::Greater => right = mid,
            }
        }
        left as i32
    }
}

fn main() {
    println!("Hello, world!");
}
