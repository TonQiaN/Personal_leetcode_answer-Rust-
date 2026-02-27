struct Solution {}

use std::cmp::Ordering;
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (mut g, mut s) = (g, s);
        g.sort_unstable();
        s.sort_unstable();

        let mut count = 0;
        let (mut g_i, mut s_i) = (g.len() as i32 - 1, s.len() as i32 - 1);
        while g_i >= 0 && s_i >= 0 {
            let (g_size, s_size) = (g[g_i as usize], s[s_i as usize]);
            match g_size.cmp(&s_size) {
                Ordering::Less | Ordering::Equal => {
                    count += 1;
                    g_i -= 1;
                    s_i -= 1;
                }
                Ordering::Greater => {
                    g_i -= 1;
                }
            }
        }

        count
    }
}

fn main() {
    println!("Hello, world!");
}
