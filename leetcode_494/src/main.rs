struct Solution {}

impl Solution {
    // pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    //     let mut ans = 0;
    //     fn backtracking(nums: &Vec<i32>, sum: i32, ans: &mut i32, i: usize, target: &i32) {
    //         if i == nums.len() {
    //             if sum == *target {
    //                 *ans += 1;
    //             }
    //             return;
    //         }
    //         let num = nums[i];
    //         backtracking(nums, sum + num, ans, i + 1, target);
    //         let num = -nums[i];
    //         backtracking(nums, sum + num, ans, i + 1, target);
    //     }
    //     backtracking(&nums, 0, &mut ans, 0, &target);
    //     ans
    // }
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        if (sum + target) % 2 == 1 || (sum + target) < 0{
            return 0;
        }
        let postive_num = ((sum + target) / 2) as usize;
        let mut dp = vec![0; postive_num + 1];
        dp[0] = 1;
        for num in nums {
            let num = num as usize;
            for i in (num..=postive_num).rev() {
                dp[i] += dp[i - num];
            }
        }

        dp[postive_num]
    }
}

fn main() {
    println!("Hello, world!");
}
