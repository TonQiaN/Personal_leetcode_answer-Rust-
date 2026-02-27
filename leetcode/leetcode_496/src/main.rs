struct Solution {}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut nums2_idx = HashMap::new();
        let mut st = vec![];
        let mut next_great_arr = vec![-1; nums2.len()];
        let mut ans = vec![-1; nums1.len()];
        for (i, &num2) in nums2.iter().enumerate() {
            nums2_idx.insert(num2, i);
            while let Some(&(i, top_num)) = st.last()
                && num2 > top_num
            {
                next_great_arr[i] = num2;
                st.pop();
            }
            st.push((i, num2));
        }
        for (j, num1) in nums1.iter().enumerate() {
            ans[j] = next_great_arr[nums2_idx[num1]];
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
