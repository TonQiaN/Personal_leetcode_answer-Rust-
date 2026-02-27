struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }
        let target = (sum / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for num in nums {
            let w = num as usize;
            if w > target {
                continue;
            }
            if dp[target - w] {return true;}
            for j in (w..=target).rev() {
                dp[j] |= dp[j - w];
            }
        }
        false
    }
    // pub fn can_partition(nums: Vec<i32>) -> bool {
    //     let sum: i32 = nums.iter().sum();
    //     if sum % 2 == 1 {
    //         return false;
    //     }
    //     let target = (sum / 2) as usize;
    //     let mut dp = vec![0; target + 1];
    //     for num in nums {
    //         if dp[target] == target as i32 {
    //             return true;
    //         }
    //         for weight in ((num as usize)..=target).rev() {
    //             dp[weight] = dp[weight].max(dp[weight - num as usize] + num);
    //         }
    //     }
    //     dp[target] == target as i32
    // }
}

fn main() {
    println!("Hello, world!");
}
