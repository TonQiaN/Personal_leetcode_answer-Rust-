struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut end_map = vec![0; 26];
        let mut ans = vec![];
        let s_bytes = s.as_bytes();
        for i in 0..26 {
            let c = b'a' + i;
            // end_map[i as usize] = s_bytes.iter().rposition(|&x| x == c).unwrap_or(0);
            end_map[i as usize] = (0..s_bytes.len()).rfind(|&x| s_bytes[x] == c).unwrap_or(0);
        }
        let mut end = end_map[(s_bytes[0] - b'a') as usize];
        let mut count = 0;
        for i in 0..s_bytes.len() {
            count += 1;
            end = end.max(end_map[(s_bytes[i] - b'a') as usize]);
            if i == end {
                ans.push(count);
                count = 0;
                continue;
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
