struct Solution {}

struct DSU {
    father: Vec<usize>,
    max_cnt: Vec<usize>,
    max_vals: Vec<usize>,
    path_num: usize,
}

impl DSU {
    fn new(vals: &Vec<i32>) -> Self {
        let mut max_vals = vec![];
        for &val in vals {
            max_vals.push(val as usize);
        }

        DSU {
            father: (0..vals.len()).collect(),
            max_cnt: vec![1; vals.len()],
            max_vals,
            path_num: 0,
        }
    }

    fn find(&mut self, id: usize) -> usize {
        if self.father[id] != id {
            self.father[id] = self.find(self.father[id]);
        }
        self.father[id]
    }

    fn union(&mut self, id1: usize, id2: usize) {
        let (fid1, fid2) = (self.find(id1), self.find(id2));
        if self.max_vals[fid1] < self.max_vals[fid2] {
            self.father[fid1] = fid2;
        } else if self.max_vals[fid2] < self.max_vals[fid1] {
            self.father[fid2] = fid1;
        } else {
            self.path_num += self.max_cnt[fid1] * self.max_cnt[fid2];
            self.father[fid1] = fid2;
            self.max_cnt[fid2] += self.max_cnt[fid1];
        }
    }
}

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut my_dsu = DSU::new(&vals);
        let mut edges = edges;
        edges.sort_by_key(|edge| i32::max(vals[edge[0] as usize], vals[edge[1] as usize]));
        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            my_dsu.union(a, b);
        }
        (vals.len() + my_dsu.path_num) as i32
    }
}

fn main() {
    println!("Hello, world!");
}
