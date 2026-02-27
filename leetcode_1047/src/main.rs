struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut ans = vec![];
        for c in s.chars() {
            match ans.last() {
                None => ans.push(c),
                Some(&last) => {
                    if last == c {
                        ans.pop();
                    } else {
                        ans.push(c);
                    }
                }
            }
        }
        ans.into_iter().collect()
    }
}

fn main() {
    println!("Hello, world!");
}
