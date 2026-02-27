struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let Some(k) = (0..nums.len() - 1).rfind(|&i| nums[i] < nums[i + 1]) else {
            nums.reverse();
            return;
        };
        let l = (k + 1..nums.len()).rfind(|&j| nums[j] > nums[k]).unwrap();
        nums.swap(k, l);
        nums[k + 1..].reverse();

        // let n = nums.len();
        // if n <= 1 {
        //     return;
        // };
        // let mut i = n - 2;
        // while i > 0 && nums[i] >= nums[i + 1] {
        //     i -= 1;
        // }
        // if i == 0 && nums[i] >= nums[i + 1] {
        //     nums.reverse();
        // } else {
        //     let mut j = n - 1;
        //     while j > i && nums[j] <= nums[i] {
        //         j -= 1;
        //     }
        //     nums.swap(i, j);
        //     nums[i + 1..].reverse();
        // }
    }
}

fn main() {
    println!("Hello, world!");
}
