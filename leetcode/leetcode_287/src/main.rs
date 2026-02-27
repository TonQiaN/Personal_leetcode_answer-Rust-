struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // let (mut slow, mut fast) = (0usize, 0usize);
        // slow = nums[slow] as usize;
        // fast = nums[nums[fast] as usize] as usize;
        // while slow != fast {
        //     slow = nums[slow] as usize;
        //     fast = nums[nums[fast] as usize] as usize;
        // }
        // fast = 0usize;
        // while slow != fast {
        //     slow = nums[slow] as usize;
        //     fast = nums[fast] as usize;
        // }
        // slow as i32

        let nums_len = nums.len();
        let (mut left, mut right) = (0usize, nums_len);
        while left < right {
            let mid = left + (right - left) / 2;
            let mut count = 0;
            for num in &nums{
                if *num as usize <= mid {
                    count += 1;
                }
            }
            if count > mid {
                right = mid;
            } else {
                left = mid + 1;
            }

        }
        left as i32
    }
}

fn main() {
    println!("Hello, world!");
}
