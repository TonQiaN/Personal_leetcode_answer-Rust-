struct Solution {}

impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut first_two = HashMap::new();
        let mut ans = 0;
        for num1 in nums1 {
            for num2 in nums2.iter() {
                *first_two.entry(num1 + num2).or_insert(0) += 1;
            }
        }
        for num3 in nums3 {
            for num4 in nums4.iter() {
                match first_two.get(&(-num3 - num4)) {
                    None => continue,
                    Some(time) => {ans += time;}
                }
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
