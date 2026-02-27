use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0usize, nums.len());
        while l < r {
            let mid = l + (r - l) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Less => {l = mid + 1},
                Ordering::Equal => {return mid as i32;},
                Ordering::Greater => {r = mid},
            }
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}
