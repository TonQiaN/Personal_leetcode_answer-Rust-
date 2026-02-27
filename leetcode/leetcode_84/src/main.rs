struct Solution {}

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights = heights;
        heights.push(0);
        heights.insert(0, 0);
        let n = heights.len();
        let mut st = vec![];
        let mut ans = 0;
        for i in 0..n {
            while !st.is_empty() && heights[*st.last().unwrap()] > heights[i] {
                let local_i = st.pop().unwrap();
                let local = heights[local_i];
                let left_i = *st.last().unwrap();
                let right_i = i;
                let h = local as usize;
                let w = right_i - left_i - 1;
                ans = ans.max(h * w);
            }
            st.push(i);
        }
        ans as i32
    }
}

fn main() {
    println!("Hello, world!");
}
