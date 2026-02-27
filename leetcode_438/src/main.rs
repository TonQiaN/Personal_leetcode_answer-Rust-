struct Solution {}

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let char_to_idx = |c: u8| (c - b'a') as usize;

        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let n = s.len();
        let m = p.len();

        let mut p_count = [0; 26];
        let mut window_count = [0; 26];

        let mut indices = vec![];
        if m > n {
            return indices;
        }

        for &c in p_bytes {
            p_count[char_to_idx(c)] += 1;
        }
        for i in 0..m {
            window_count[char_to_idx(s_bytes[i])] += 1;
        }
        if p_count == window_count {
            indices.push(0);
        }
        for i in m..n {
            window_count[char_to_idx(s_bytes[i])] += 1;
            window_count[char_to_idx(s_bytes[i - m])] -= 1;
            if p_count == window_count {
                indices.push((i + 1 - m) as i32);
            }
        }

        indices
    }
}


fn main() {
    println!("Hello, world!");
}
