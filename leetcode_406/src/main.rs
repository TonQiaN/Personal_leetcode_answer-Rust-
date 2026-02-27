use std::cmp::Reverse;

struct Solution {}

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        let mut ans = Vec::with_capacity(people.len());
        // people.sort_by(|a, b| b[0].cmp(&a[0]).then_with(|| a[1].cmp(&b[1])));
        // people.sort_by_key(|p| (Reverse(p[0]), p[1]));
        people.sort_by(|a,b|{
            match b[0].cmp(&a[0]) {
                std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
                other => other,
            }
        });
        for p in people {
            ans.insert(p[1] as usize, p);
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
