struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut path = vec![];
        let mut ans = vec![];

        fn backtracking(s: &str, start: usize, path: &mut Vec<String>, ans: &mut Vec<String>) {
            if path.len() == 4 {
                if start == s.len() {
                    ans.push(path.join("."));
                }
                return;
            }
            
            let end = (start + 3).min(s.len());
            for i in start + 1..=end {
                let new_num = &s[start..i];
                if (new_num.len() > 1 && new_num.starts_with('0')) || new_num.parse::<u8>().is_err() {continue;}
                path.push(new_num.to_string());
                backtracking(s, i, path, ans);
                path.pop();
            }
        }
        backtracking(&s, 0, &mut path, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
