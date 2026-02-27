
struct Solution {}

// impl Solution {
//     fn find_next(ends: &Vec<i32>, target: i32) -> usize {
//         use std::cmp::Ordering;
//         let (mut l, mut r) = (0usize, ends.len());
//         while l < r {
//             let mid = l + (r - l) / 2;
//             match ends[mid].cmp(&target) {
//                 Ordering::Less => {l = mid + 1;},
//                 Ordering::Equal => {return mid;},
//                 Ordering::Greater => {r = mid;},
//             };
//         }
//         l
//     }
//     pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//         // let mut dp = vec![1; nums.len()];
//         let mut ends = vec![];
//         // let mut ans = 0;
//         for i in 0..nums.len() {
//             let next = Self::find_next(&ends, nums[i]);
//             if next == ends.len() {
//                 ends.push(nums[i]);
//             } else {
//                 ends[next] = nums[i];
//             }
//             // dp[i] = next + 1;
//             // ans = ans.max(dp[i]);
//         }
//         // ans as i32
//         ends.len() as i32
//     }
//     // pub fn length_of_lis(nums: Vec<i32>) -> i32 {
//     //     let mut dp = vec![1;nums.len()];
//     //     let mut ans = 0;
//     //     for i in 0..nums.len() {
//     //         for j in 0..i {
//     //             if nums[i] > nums[j] {
//     //                 dp[i] = dp[i].max(dp[j] + 1);
//     //             }
//     //         }
//     //         ans = ans.max(dp[i]);
//     //     }
//     //     ans
//     // }
// }

impl Solution {
    pub fn length_of_lis(mut nums: Vec<i32>) -> i32 {
        let mut ends: Vec<i32> = Vec::new();
        for x in nums.into_iter() {
            // 第一个 >= x 的位置
            let i = ends.partition_point(|&val| val <  x); 
            if i == ends.len() {
                ends.push(x);
            } else {
                ends[i] = x;
            }
        }
        ends.len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
