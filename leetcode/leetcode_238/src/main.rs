use std::iter::Product;

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1;n];
        // let mut first_curr = 1;
        // answer[0] = first_curr;
        // if let Some((_, prev)) = nums.split_last() {
        //     for (i, num) in prev.iter().enumerate() {
        //         answer[i + 1] = first_curr * num;
        //         first_curr *= num;
        //     }
        // }
        let mut prev = 1;
        for i in 0..n {
            answer[i] = prev;
            prev *= nums[i];
        }

        let mut rev_prev = 1;
        for i in (0..n).rev() {
            answer[i] *= rev_prev;
            rev_prev *= nums[i];
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
    Solution::product_except_self(vec![1, 2, 3, 4]);
}
