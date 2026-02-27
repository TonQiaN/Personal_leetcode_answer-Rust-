use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn sort_to_find(nums: &mut Vec<i32>, k: i32, mut lt: usize, mut gt: usize) -> i32 {
        let mut i = lt;
        let n = nums.len();
        let mid = lt + (gt - lt) / 2;
        let mid_num = nums[mid];
        let (lt_back, gt_back) = (lt, gt);
        while i < gt {
            match nums[i].cmp(&mid_num) {
                Ordering::Less => {
                    nums.swap(i, lt);
                    i += 1;
                    lt += 1;
                }
                Ordering::Equal => i += 1,
                Ordering::Greater => {
                    gt -= 1;
                    nums.swap(i, gt);
                }
            }
        }
        let t = n - k as usize;
        match () {
            _ if t < lt => Self::sort_to_find(nums, k, lt_back, lt),
            _ if t > gt - 1 => Self::sort_to_find(nums, k, gt, gt_back),
            _ => nums[n - k as usize],
        }
    }

    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let (lt, gt) = (0usize, n);
        Self::sort_to_find(&mut nums, k, lt, gt)
    }
}
