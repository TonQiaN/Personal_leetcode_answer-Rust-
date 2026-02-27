struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        // let mut s = s.chars().collect::<Vec<_>>();
        // s.reverse();
        // let mut slow = 0;
        // for i in 0..s.len() {
        //     if s[i] == ' ' && i > 0{
        //         s[slow..i].reverse();
        //         slow = i + 1;
        //     }
        // }
        // s[slow..].reverse();
        // let mut slow = 0;
        // let mut count = 0;
        // for i in 0..s.len() {
        //     if s[i] == ' ' {
        //         count += 1;
        //     } else {
        //         count = 0;
        //     }
        //     if s[i] == ' ' && count > 1 {
        //         continue;
        //     }
        //     s[slow] = s[i];
        //     slow += 1;
        // }
        // if s[slow] == ' ' {
        //     slow -= 1;
        // }
        // let mut start = 0;
        // if s[start] == ' ' {
        //     start += 1;
        // }
        // s[start..slow].into_iter().collect()
            s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}

fn main() {
    println!("Hello, world!");
}
