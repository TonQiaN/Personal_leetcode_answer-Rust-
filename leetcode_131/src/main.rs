struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut ans = vec![];
        let mut path = vec![];

        fn is_palindrome(s: &[u8]) -> bool {
            if s.len() == 1 {
                return true;
            }
            (0..s.len() / 2).all(|i| s[i] == s[s.len() - i - 1])
        }

        fn backtracking(
            s: &[u8],
            start: usize,
            path: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
        ) {
            if start == s.len() {
                ans.push(path.clone());
                return;
            }
            for end in start..s.len() {
                let new_str = &s[start..=end];
                if is_palindrome(new_str) {
                    path.push(String::from_utf8(new_str.to_vec()).unwrap());
                    backtracking(s, end + 1, path, ans);
                    path.pop();
                }
            }
        }
        backtracking(s.as_bytes(), 0, &mut path, &mut ans);
        ans
    }
}

// impl Solution {
//     pub fn partition(s: String) -> Vec<Vec<String>> {
//         let mut ans = vec![];
//         let mut path = vec![];
//         let mut cur_str = String::new();

//         fn is_palindrome(s: &str) -> bool {
//             let s = s.chars().collect::<Vec<_>>();
//             let (mut start, mut end) = (0, s.len() - 1);
//             while start < end {
//                 if s[start] != s[end] {
//                     return false;
//                 }
//                 start += 1;
//                 end -= 1;
//             }
//             true
//         }

//         fn backtracking(
//             s: &str,
//             cur_str: &mut String,
//             path: &mut Vec<String>,
//             ans: &mut Vec<Vec<String>>,
//         ) {
//             if s.is_empty() {
//                 ans.push(path.clone());
//                 return;
//             }
//             for (i, c) in s.chars().enumerate() {
//                 cur_str.push(c);
//                 if is_palindrome(cur_str) {
//                     path.push(cur_str.clone());
//                     let saved = std::mem::take(cur_str);
//                     backtracking(&s[i + 1..], cur_str, path, ans);
//                     *cur_str = saved;
//                     path.pop();
//                 }
//             }
//         }
//         backtracking(&s, &mut cur_str, &mut path, &mut ans);
//         ans
//     }
// }

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut dp = vec![vec![false; n]; n];
        let mut ans = vec![];
        let mut path = vec![];
        for i in (0..n).rev() {
            for j in i..n {
                if s_bytes[i] == s_bytes[j] && (j - i <= 2 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                }
            }
        }

        fn backtracking(
            s_bytes: &[u8],
            start: usize,
            path: &mut Vec<String>,
            ans: &mut Vec<Vec<String>>,
            dp: &Vec<Vec<bool>>,
        ) {
            if start == s_bytes.len() {
                ans.push(path.clone());
                return;
            }
            for end in start..s_bytes.len() {
                if dp[start][end] {
                    let word = String::from_utf8(s_bytes[start..=end].to_vec()).unwrap();
                    path.push(word);
                    backtracking(s_bytes, end + 1, path, ans, dp);
                    path.pop();
                }
            }
        }

        backtracking(&s_bytes, 0, &mut path, &mut ans, &dp);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
