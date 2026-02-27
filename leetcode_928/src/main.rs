struct Solution {}

struct DSU {
    father: Vec<usize>,
    size: Vec<usize>,
    infect: Vec<i32>,
    virtus: Vec<bool>,
    cnts: Vec<usize>,
}

impl DSU {
    fn new(n: usize, initial: &Vec<i32>) -> Self {
        let mut virtus = vec![false; n];
        for &i in initial {
            virtus[i as usize] = true;
        }
        DSU {
            father: (0..n).collect(),
            size: vec![1; n],
            infect: vec![-1; n],
            virtus,
            cnts: vec![0; n],
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
            self.father[fid1] = fid2;
            self.size[fid2] += self.size[fid1];
        }
    }
}

impl Solution {
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let n = graph.len();
        let mut initial = initial;
        initial.sort();
        let mut my_dsu = DSU::new(n, &initial);
        for i in 0..n {
            for j in (i + 1)..n {
                if graph[i][j] == 1 && !my_dsu.virtus[i] && !my_dsu.virtus[j] {
                    my_dsu.union(i, j);
                }
            }
        }

        for &i in &initial {
            for j in 0..n {
                if j != i as usize && !my_dsu.virtus[j] && graph[i as usize][j] == 1 {
                    let f_neighbour = my_dsu.find(j);
                    if my_dsu.infect[f_neighbour] == -1 {
                        my_dsu.infect[f_neighbour] = i;
                    } else if my_dsu.infect[f_neighbour] != i && my_dsu.infect[f_neighbour] != -2 {
                        my_dsu.infect[f_neighbour] = -2;
                    }
                }
            }
        }

        for i in 0..n {
            if i == my_dsu.find(i) && my_dsu.infect[i] != -1 && my_dsu.infect[i] != -2 {
                my_dsu.cnts[my_dsu.infect[i] as usize] += my_dsu.size[i];
            }
        }

        let mut ans = initial[0];
        let mut cnt = my_dsu.cnts[ans as usize];
        for i in initial {
            if my_dsu.cnts[i as usize] > cnt {
                ans = i;
                cnt = my_dsu.cnts[i as usize];
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
