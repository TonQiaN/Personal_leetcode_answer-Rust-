
use std::cmp::Ordering;
struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let (mut l, mut r) = (0usize, nums.len());
        let mut ans = Vec::with_capacity(nums.len());
        for k in (0..nums.len()).rev() {
            match (nums[l] * nums[l]).cmp(&(nums[r - 1] * nums[r - 1])) {
                Ordering::Less => {
                    ans[k] = nums[r - 1] * nums[r - 1];
                    r -= 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    ans[k] = nums[l] * nums[l];
                    l += 1;
                }
            }
        }
        ans
        // let mut pos = (0..nums.len())
        //     .find(|&x| nums[x] >= 0)
        //     .unwrap_or(nums.len());
        // if pos == 0 {
        //     return nums.into_iter().map(|x| x * x).collect();
        // } else if pos == nums.len() {
        //     return nums.into_iter().rev().map(|x| x * x).collect();
        // } else {
        //     let mut neg: i32 = (pos - 1) as i32;
        //     let mut ans = vec![];
        //     while pos < nums.len() && neg >= 0 {
        //         if nums[pos] * nums[pos] < nums[neg as usize] * nums[neg as usize] {
        //             ans.push(nums[pos] * nums[pos]);
        //             pos += 1;
        //         } else {
        //             ans.push(nums[neg as usize] * nums[neg as usize]);
        //             neg -= 1;
        //         }
        //     }
        //     while pos < nums.len() {
        //         ans.push(nums[pos] * nums[pos]);
        //         pos += 1;
        //     }
        //     while neg >= 0 {
        //         ans.push(nums[neg as usize] * nums[neg as usize]);
        //         neg -= 1;
        //     }
        //     return ans;
        // }
    }
}
fn main() {
    println!("Hello, world!");
}
