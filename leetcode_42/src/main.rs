struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n == 1 {
            return 0;
        }
        let mut ans = 0;
        let (mut l, mut r) = (1, n - 2);
        let (mut l_max, mut r_max) = (height[0], height[n - 1]);
        while l <= r {
            if l_max < r_max {
                ans += i32::max(0, l_max - height[l]);
                l_max = l_max.max(height[l]);
                l += 1;
            } else {
                ans += i32::max(0, r_max - height[r]);
                r_max = r_max.max(height[r]);
                r -= 1;
            }
        }
        ans
    }
}

// impl Solution {
//     pub fn trap(height: Vec<i32>) -> i32 {
//         let mut st = vec![];
//         let n = height.len();
//         let mut ans = 0;
//         for i in 0..n {
//             while let Some(last_i) = st.last() && height[*last_i] <= height[i] {
//                 if st.len() == 1 {
//                     st.pop();
//                 } else {
//                     let local_i = st.pop().unwrap();
//                     let local = height[local_i];
//                     let left = height[*st.last().unwrap()];
//                     let right = height[i];
//                     let h = i32::min(left, right) - local;
//                     let w = i - *st.last().unwrap() - 1;
//                     ans += h * w as i32;
//                 }
//             }
//             st.push(i);
//         }
//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
