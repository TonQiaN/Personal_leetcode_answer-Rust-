struct Solution {}

struct DSU {
    father: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            father: (0..n).collect(),
            size: vec![1; n],
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
        }
    }

    fn reset(&mut self, id: usize) {
        self.father[id] = id;
        self.size[id] = 1;
    }
}

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {

        let n = n as usize;
        let mut my_dsu = DSU::new(n);
        let mut ans = vec![];
        let mut meetings = meetings;
        meetings.sort_by_key(|meeting| meeting[2]);
        my_dsu.union(first_person as usize, 0);
        let mut current_time = -1;
        let mut person_set = vec![];
        for meeting in meetings {
            let (a, b, time) = (meeting[0], meeting[1], meeting[2]);
            if current_time != time {
                person_set.sort_unstable();
                person_set.dedup();
                for &person in &person_set {
                    let root_person = my_dsu.find(person);
                    let root_0 = my_dsu.find(0);
                    if root_0 != root_person {
                        my_dsu.reset(person);
                    }
                }
                person_set.clear();
                current_time = time;
            }
            person_set.push(a as usize);
            person_set.push(b as usize);
            my_dsu.union(a as usize, b as usize);
        }
        
        person_set.sort_unstable();
        person_set.dedup();
        for &person in &person_set {
            let root_person = my_dsu.find(person);
            let root_0 = my_dsu.find(0);
            if root_0 != root_person {
                my_dsu.reset(person);
            }
        }

        for i in 0..n {
            let root_i = my_dsu.find(i);
            let root_0 = my_dsu.find(0);
            if root_0 == root_i {
                ans.push(i as i32);
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
