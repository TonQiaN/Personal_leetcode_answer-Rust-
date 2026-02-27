struct Solution {}

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut k = k;
        let mut count = 0;
        nums.sort();
        for i in 0..nums.len() {
            if k == 0 {
                break;
            }
            if nums[i] < 0 {
                nums[i] = -nums[i];
                k -= 1;
                count += 1;
            }
        }
        let mut sum = nums.iter().sum();
        if k % 2 == 1 {
            sum -= 2 * nums.iter().min().unwrap();
        }
        sum
    }
}

fn main() {
    println!("Hello, world!");
}
