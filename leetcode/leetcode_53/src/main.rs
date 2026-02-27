use std::i32;


struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut sum = 0;
        for num in nums {
            sum = i32::max(num, sum + num);
            ans = ans.max(sum);
        }
        ans
    }
}
// impl Solution {
//     pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//         let mut ans = i32::MIN;
//         let mut sum = 0;
//         let mut min_pre = 0;
//         for num in nums {
//             sum += num;
//             ans = ans.max(sum - min_pre);
//             min_pre = min_pre.min(sum)
//         }
//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
