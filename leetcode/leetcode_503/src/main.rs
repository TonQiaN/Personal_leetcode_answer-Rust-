struct Solution {}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        let mut st = vec![];
        for i in 0..2 * n {
            let new_i = i % n;
            while let Some(last_i) = st.last()
                && nums[*last_i] < nums[new_i]
            {
                ans[*last_i] = nums[new_i];
                st.pop();
            }
            if i < n {
                st.push(i);
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
