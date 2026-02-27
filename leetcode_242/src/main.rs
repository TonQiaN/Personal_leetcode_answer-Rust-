struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut expected = vec![0; 26];
        let mut target = vec![0; 26];
        s.as_bytes().iter().for_each(|&c| expected[(c - b'a') as usize] += 1);
        t.as_bytes().iter().for_each(|&c| target[(c - b'a') as usize] += 1);
        target == expected
    }
}

fn main() {
    println!("Hello, world!");
}
