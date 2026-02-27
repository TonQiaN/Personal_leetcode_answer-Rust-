struct Solution {}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // let s_bytes = s.as_bytes();
        // let mut dp_arr = [-1;101];
        // fn dp(s_bytes: &[u8], i: usize, dp_arr: &mut [i32]) -> i32 {
        //     let n = s_bytes.len();
        //     if i == n {
        //         return 1;
        //     }
        //     if dp_arr[i] != -1 {
        //         return dp_arr[i];
        //     }
        //     if s_bytes[i] == b'0' {
        //         return 0;
        //     } else {
        //         let mut ans = dp(s_bytes, i + 1, dp_arr);
        //         if i + 1 < n {
        //             let num = (s_bytes[i] - b'0') * 10 + (s_bytes[i + 1] - b'0');
        //             if (10..=26).contains(&num) {
        //                 ans += dp(s_bytes, i + 2, dp_arr);
        //             }
        //         }
        //         dp_arr[i] = dp_arr[i].max(ans);
        //         return ans
        //     }
        // }
        // dp(s_bytes, 0, &mut dp_arr)


        // let s_bytes = s.as_bytes();
        // let n = s_bytes.len();
        // let mut dp_arr = [-1;101];
        // dp_arr[n] = 1;
        // for i in (0..n).rev() {
        //     if s_bytes[i] == b'0' {
        //         dp_arr[i] = 0;
        //         continue
        //     }
        //     let mut ans = dp_arr[i + 1];
        //     if i + 1 < n {
        //         let two_num = (s_bytes[i] - b'0') * 10 + (s_bytes[i + 1] - b'0');
        //         if (10..=26).contains(&two_num) {
        //             ans += dp_arr[i + 2];
        //         }
        //     }
        //     dp_arr[i] = dp_arr[i].max(ans);
        // }
        // dp_arr[0]

        let s_bytes = s.into_bytes();
        s_bytes.iter().rfold(
            (1, 0, None::<u8>),
            |(dp1, dp2, next), &c|
            {
                let mut curr = if c == b'0' {0} else {dp1};
                if let Some(next_num) = next {
                    let two_num = (c - b'0') * 10 + (next_num - b'0');
                    if two_num <= 26 && two_num >= 10 {
                        curr += dp2;
                    }
                }
                (curr, dp1, Some(c))
            }
        ).0
    }
}

fn main() {
    println!("Hello, world!");
}
