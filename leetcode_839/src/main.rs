struct Solution {}

struct DSU {
    father: Vec<usize>,
    size: Vec<usize>,
    groups_number: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            father: (0..n).collect(),
            size: vec![1; n],
            groups_number: n,
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
        if fid1 != fid2 {
            if self.size[fid1] < self.size[fid2] {
                self.father[fid1] = fid2;
                self.size[fid2] += self.size[fid1];
            } else {
                self.father[fid2] = fid1;
                self.size[fid1] += self.size[fid2];
            }
            self.groups_number -= 1;
        }
    }
}

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        fn is_similar(str1: &str, str2: &str) -> bool {
            let diff_num = str1
                .as_bytes()
                .into_iter()
                .zip(str2.as_bytes().into_iter())
                .filter(|&(a, b)| a != b)
                .count();
            diff_num == 0 || diff_num == 2
        }

        let n = strs.len();
        let mut my_dsu = DSU::new(n);
        for i in 0..n {
            for j in i + 1..n {
                if is_similar(&strs[i], &strs[j]) {
                    my_dsu.union(i, j);
                }
            }
        }
        my_dsu.groups_number as i32
    }
}

fn main() {
    println!("Hello, world!");
}
