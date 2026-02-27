struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // s.reverse();
        let (mut i, mut j) = (0usize, s.len());
        while i < j {
            s.swap(i, j - 1);
            i += 1;
            j -= 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
