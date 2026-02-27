struct Solution {}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        struct DisjointSet {
            father: Vec<usize>,
            size: Vec<usize>,
            lands: usize,
        }

        impl DisjointSet {
            fn new(n: usize) -> Self {
                DisjointSet {
                    father: (0..n).collect(),
                    size: vec![1; n],
                    lands: n,
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
                        self.lands -= 1;
                    } else {
                        self.father[fid2] = fid1;
                        self.size[fid1] += self.size[fid2];
                        self.lands -= 1;
                    }
                }
            }
        }

        let n = grid.len();
        let m = grid.first().map_or(0, |r0| r0.len());
        let mut my_dsu = DisjointSet::new(n * m);
        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == '1' {
                    let id1 = r * m + c;
                    let neighbours = [
                        (r.wrapping_sub(1), c),
                        (r, c.wrapping_sub(1)),
                    ];
                    for (r1, c1) in neighbours {
                        if r1 < n && c1 < m && grid[r1][c1] == '1' {
                            let id2 = r1 * m + c1;
                            my_dsu.union(id1, id2);
                        }
                    }
                } else {
                    my_dsu.lands -= 1;
                }
            } 
        }
        my_dsu.lands as i32 
    }
}

// impl Solution {
//     pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
//         let mut ans = 0;
//         let (n, m) = (grid[0].len(), grid.len());
//         let mut used = vec![vec![false; n]; m];

//         fn label(grid: &Vec<Vec<char>>, i: usize, j: usize, used: &mut Vec<Vec<bool>>) {
//             used[i][j] = true;
//             if i + 1 < grid.len() && grid[i + 1][j] == '1' && !used[i + 1][j] {
//                 label(grid, i + 1, j, used);
//             }
//             if j + 1 < grid[0].len() && grid[i][j + 1] == '1' && !used[i][j + 1] {
//                 label(grid, i, j + 1, used);
//             }
//             if i > 0 && grid[i - 1][j] == '1' && !used[i - 1][j] {
//                 label(grid, i -  1, j, used);
//             }
//             if j > 0 && grid[i][j - 1] == '1' && !used[i][j - 1] {
//                 label(grid, i, j - 1, used);
//             }
//         }

//         for i in 0..m {
//             for j in 0..n {
//                 if grid[i][j] == '1' && !used[i][j] {
//                     ans += 1;
//                     label(&grid, i, j, &mut used);
//                 }
//             }
//         }
//         ans
//     }
// }

fn main() {
    println!("Hello, world!");
}
