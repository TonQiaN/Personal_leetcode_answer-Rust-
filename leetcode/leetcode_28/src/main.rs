struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let needle = needle.as_bytes();
        let haystack = haystack.as_bytes();
        let mut next = vec![0; needle.len()];
        let mut j = 0;
        for i in 1..needle.len() {
            while j > 0 && needle[j] != needle[i] {
                j = next[(j - 1) as usize];
            }
            if needle[j] == needle[i] {
                j += 1;
            }
            next[i] = j;
        }
        
        let mut j = 0;
        for i in 0..haystack.len() {
            while j > 0 && haystack[i] != needle[j] {
                j = next[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == needle.len() {
                return (i - needle.len() + 1) as i32
            }
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
    println!("{}", Solution::str_str("aabaabaaf".to_string(), "aabaaf".to_string()));
}
