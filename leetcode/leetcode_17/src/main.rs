struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let mut ans = vec![];
        let mut path = String::new();
        let num_to_str = [
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];

        fn back_tracking(
            digits: &str,
            path: &mut String,
            ans: &mut Vec<String>,
            num_to_str: &[&'static str; 10],
        ) {
            if digits.is_empty() {
                ans.push(path.clone());
                return;
            }
            let idx = digits.chars().next().unwrap().to_digit(10).unwrap() as usize;
            for char in num_to_str[idx].chars() {
                path.push(char);
                back_tracking(&digits[1..], path, ans, num_to_str);
                path.pop();
            }
        }
        back_tracking(&digits, &mut path, &mut ans, &num_to_str);
        ans
    }
}

// impl Solution {
//     pub fn letter_combinations(digits: String) -> Vec<String> {
//         let mut ans = vec![];
//         let mut path = String::new();
//         let num_to_str = vec![
//             "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
//         ];

//         fn back_tracking(
//             digits: &str,
//             path: &mut String,
//             ans: &mut Vec<String>,
//             num_to_str: &Vec<&'static str>,
//             index: usize,
//         ) {
//             if index == digits.len() {
//                 ans.push(path.clone());
//                 return;
//             }
//             let idx = digits.chars().nth(index).unwrap().to_digit(10).unwrap() as usize;
//             for char in num_to_str[idx].chars() {
//                 path.push(char);
//                 back_tracking(digits, path, ans, num_to_str, index + 1);
//                 path.pop();
//             }
//         }
//         back_tracking(&digits, &mut path, &mut ans, &num_to_str, 0);
//         ans
//     }
// }

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans = vec![];
        let mut path = String::new();
        let digits_bytes = digits.as_bytes();
        let digit_to_string = ["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

        fn backtracking(
            digits_bytes: &[u8],
            digit_to_string: &[&str],
            start: usize,
            path: &mut String,
            ans: &mut Vec<String>,
        ) {
            if start == digits_bytes.len() {
                ans.push(path.clone());
                return;
            }
            let digit = (digits_bytes[start] - b'0') as usize;
            for word in digit_to_string[digit].chars() {
                path.push(word);
                backtracking(digits_bytes, digit_to_string, start + 1, path, ans);
                path.pop();
            }
        }
        backtracking(&digits_bytes, &digit_to_string, 0, &mut path, &mut ans);

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
