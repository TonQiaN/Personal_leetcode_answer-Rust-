struct Solution {}

use std::collections::HashSet;
struct DSU {
    father: Vec<usize>,
    size: Vec<usize>,
    groups: usize,
    active_map: HashSet<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            father: (0..n).collect(),
            size: vec![1; n],
            groups: 0,
            active_map: HashSet::new(),
        }
    }

    fn find(&mut self, id: usize) -> usize {
        if self.father[id] != id {
            self.father[id] = self.find(self.father[id]);
        }
        self.father[id]
    }

    fn active(&mut self, id:usize) {
        if self.active_map.insert(id) {
            self.groups += 1;
        }
    }

    fn union(&mut self, id1: usize, id2: usize) {
        let (fid1, fid2) = (self.find(id1), self.find(id2));
        self.active(id1);
        self.active(id2);
        if fid1 != fid2 {
            if self.size[fid1] < self.size[fid2] {
                self.father[fid1] = fid2;
                self.size[fid2] += self.size[fid1];
            } else {
                self.father[fid2] = fid1;
                self.size[fid1] += self.size[fid2];
            }
            self.groups -= 1;
        }
    }

}

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let offset = 10001;
        let mut my_dsu = DSU::new(20002);
        for stone in &stones {
            let (r, c) = (stone[0], stone[1] + offset);
            my_dsu.union(r as usize, c as usize);
        }
        (stones.len() - my_dsu.groups) as i32
    }
}

fn main() {
    println!("Hello, world!");
}
