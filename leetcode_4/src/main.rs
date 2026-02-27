struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() < nums2.len() {(nums1, nums2)} else {(nums2, nums1)};
        let m = nums1.len();
        let n = nums2.len();
        
        let total_left = (n + m + 1) / 2;
        let mut left = 0;
        let mut right = m;
        
        while left < right {
            let i = left + (right - left + 1) / 2;
            let j = total_left - i;
            if nums1[i - 1] > nums2[j] {
                right = i - 1;
            } else {
                left = i;
            }
        }
        let (i, j) = (left, total_left - left);
        let nums1_left = if i == 0 {f64::MIN} else {nums1[i - 1] as f64};
        let nums1_right = if i == m {f64::MAX} else {nums1[i] as f64};
        let nums2_left = if j == 0 {f64::MIN} else {nums2[j - 1] as f64};
        let nums2_right = if j == n {f64::MAX} else {nums2[j] as f64};

        if (n + m) % 2 == 0 {
            (f64::max(nums1_left, nums2_left) + f64::min(nums1_right, nums2_right)) / 2f64
        } else {
            f64::max(nums1_left, nums2_left)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
