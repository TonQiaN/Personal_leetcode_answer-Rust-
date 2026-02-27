struct Solution {}

struct DSU {
    father: Vec<usize>,
    size: Vec<usize>,
    set_number: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            father: (0..n).collect(),
            size: vec![1; n],
            set_number: n,
        }
    }

    fn find(&mut self, id: usize) -> usize {
        if self.father[id] != id {
            self.father[id] = self.find(self.father[id]);
        }
        self.father[id]
    }

    fn union(&mut self, id1: usize, id2: usize) {
        let fid1 = self.find(id1);
        let fid2 = self.find(id2);
        if fid1 != fid2 {
            if self.size[fid1] < self.size[fid2] {
                self.father[fid1] = fid2;
                self.size[fid2] += self.size[fid1];
            } else {
                self.father[fid2] = fid1;
                self.size[fid1] += self.size[fid2];
            }
            self.set_number -= 1;
        }
    }
}

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let mut my_dsu = DSU::new(row.len() / 2);
        for i in (0..row.len()).step_by(2) {
            my_dsu.union(row[i] as usize / 2, row[i + 1] as usize / 2);
        }
        row.len() as i32 / 2 - my_dsu.set_number as i32
    }
}

fn main() {
    println!("Hello, world!");
}
