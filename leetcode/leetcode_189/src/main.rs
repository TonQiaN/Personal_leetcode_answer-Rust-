struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let k = (k as usize) % length;
        // let copy = nums.clone();
        // for i in 0..length {
        //     nums[(i + k) % length] = copy[i];
        // }
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

fn main() {
    println!("Hello, world!");
}
