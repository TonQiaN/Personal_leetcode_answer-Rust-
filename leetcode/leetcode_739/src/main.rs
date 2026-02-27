struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut st = vec![];
        let mut ans = vec![0; temperatures.len()];
        for i in 0..temperatures.len() {
            while let Some(last_i) = st.last() && temperatures[*last_i] < temperatures[i] {
                ans[*last_i] = (i - last_i) as i32;
                st.pop();
            }
            st.push(i);
        }
        ans
    }
}

// impl Solution {
//     pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//         let mut st = vec![];
//         let n = temperatures.len();
//         let mut ans = vec![0; n];
//         for i in 0..n {
//             while let Some(last_i) = st.last() && temperatures[i] > temperatures[*last_i]{
//                 ans[*last_i] = (i - last_i) as i32;
//                 st.pop();
//             }
//             st.push(i); 
//         }
//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
