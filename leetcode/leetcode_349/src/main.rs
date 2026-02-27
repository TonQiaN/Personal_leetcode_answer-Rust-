struct Solution {}

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let nums1_set:HashSet<_> = nums1.into_iter().collect();
        let nums2_set:HashSet<_> = nums2.into_iter().collect();
        nums1_set.intersection(&nums2_set).copied().collect()
    }   
}

fn main() {
    println!("Hello, world!");
}
