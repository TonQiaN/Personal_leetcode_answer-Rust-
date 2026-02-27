
struct Solution {}

use std::cmp::Ordering;
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut pre_diff= 0;
        for i in 1..nums.len() {
            let cur_diff = nums[i] - nums[i - 1];
            if (pre_diff >= 0 && cur_diff < 0) || (pre_diff <= 0 && cur_diff > 0){
                count += 1;
                pre_diff = cur_diff;
            }
        }
        count
    }
}
// impl Solution {
//     pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
//         let mut count = 0;
//         let mut pre_diff= 0;
//         let mut cur_diff = 0;
//         for i in 0..nums.len() {
//             if i == 0 {
//                 count += 1;
//                 continue;
//             }

//             if i == 1 {
//                 if nums[i] != nums[i - 1] {
//                     count += 1;
//                 }
//                 pre_diff = nums[i] - nums[i - 1];
//                 continue;
//             }

//             cur_diff = nums[i] - nums[i - 1];
            
//             match (pre_diff.cmp(&0), cur_diff.cmp(&0)) {
//                 (Ordering::Less, Ordering::Greater) => count += 1,
//                 (Ordering::Greater, Ordering::Less) => count += 1,
//                 (Ordering::Equal, Ordering::Less) => count += 1,
//                 (Ordering::Equal, Ordering::Greater) => count += 1,
//                 _ => {},
//             }

//             if cur_diff != 0 {
//                 pre_diff = cur_diff ;
//             }
//         }

//         count
//     }
// }

fn main() {
    println!("Hello, world!");
}
