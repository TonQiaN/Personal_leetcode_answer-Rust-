struct Solution {}

struct UnionFold {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFold {
    fn new(n: usize) -> Self {
        UnionFold {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn join(&mut self, x: usize, y: usize) {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx == fy {
            return;
        }
        if self.size[fx] < self.size[fy] {
            self.parent[fx] = fy;
            self.size[fy] += self.size[fx];
        } else {
            self.parent[fy] = fx;
            self.size[fx] += self.size[fx];
        }
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut my_union = UnionFold::new(n as usize);
        for edge in edges {
            my_union.join(edge[0] as usize, edge[1] as usize);
        }
        my_union.connected(source as usize, destination as usize)
    }
}

fn main() {
    println!("Hello, world!");
}
