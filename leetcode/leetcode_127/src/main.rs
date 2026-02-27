struct Solution {}

// use std::collections::{btree_map::OccupiedEntry, HashMap, HashSet, VecDeque};
// impl Solution {
//     pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
//         // worl_list存入set, 如果目标单词不在set,说明无解
//         let word_set: HashSet<String> = word_list.into_iter().collect();
//         if !word_set.contains(&end_word) {
//             return 0;
//         }

//         let ret = Solution::bfs(begin_word, end_word, word_set);
//         match ret {
//             -1 => 0,
//             _ => ret + 1,
//         }
//     }
//     // 双向bfs
//     pub fn bfs(begin_word: String, end_word: String, word_set: HashSet<String>) -> i32 {
//         let mut d1: VecDeque<String> = VecDeque::new(); // 代表从起点开始搜索 begin_word
//         let mut d2: VecDeque<String> = VecDeque::new(); // 代表从终点开始搜索 end_word

//         /*
//          * m1 和 m2 分别记录两个方向出现的单词是经过多少次转换而来
//          * e.g.
//          * m1 = {"abc":1} 代表 abc 由 beginWord 替换 1 次字符而来
//          * m2 = {"xyz":3} 代表 xyz 由 endWord 替换 3 次字符而来
//          */
//         let mut m1: HashMap<String, i32> = HashMap::new();
//         let mut m2: HashMap<String, i32> = HashMap::new();

//         d1.push_back(begin_word.clone());
//         m1.insert(begin_word, 0);

//         d2.push_back(end_word.clone());
//         m2.insert(end_word, 0);

//         /*
//          * 只有两个队列都不空，才有必要继续往下搜索
//          * 如果其中一个队列空了，说明从某个方向搜到底都搜不到该方向的目标节点
//          * e.g.
//          * 例如，如果 d1 为空了，说明从 beginWord 搜索到底都搜索不到 endWord，反向搜索也没必要进行了
//          */
//         while !d1.is_empty() && !d2.is_empty() {
//             let t;
//             // 为了让两个方向的搜索尽可能平均，优先拓展队列内元素少的方向
//             if d1.len() <= d2.len() {
//                 t = Self::update(&mut d1, &mut m1, &mut m2, &word_set);
//             } else {
//                 t = Self::update(&mut d2, &mut m2, &mut m1, &word_set);
//             }

//             if t != -1 {
//                 return t;
//             }
//         }

//         -1
//     }
//     /// update 代表从 deque 中取出一个单词进行扩展，
//     /// cur: 当前方向的距离词典
//     /// other: 另一个方向的距离词典
//     pub fn update(
//         deque: &mut VecDeque<String>,
//         cur: &mut HashMap<String, i32>,
//         other: &mut HashMap<String, i32>,
//         word_set: &HashSet<String>,
//     ) -> i32 {
//         let m = deque.len();

//         for _ in 0..m {
//             // 获取需要扩展的原字符串
//             let poll = deque.pop_front().expect("empty queue");
//             let n = poll.len();

//             // 替换poll中的每个字符为a..z
//             for i in 0..n {
//                 for j in 'a'..='z' {
//                     let mut sub = poll.clone().into_bytes();
//                     sub[i] = j as u8;
//                     // 替换1个字母后的字符串
//                     let sub_str = String::from_utf8(sub).expect("utf8 string error");
//                     if word_set.contains(&sub_str) {
//                         // 如果该字符串在「当前方向」被记录过（拓展过），跳过即可
//                         // cur[&sub_str] <= cur[&poll] + 1 的含义:
//                         // 如果 sub_str 已经出现在 cur 字典中，并且其记录的步数 cur[&sub_str] 小于等于 cur[&poll] + 1
//                         // 这意味着我们发现的到达 sub_str 的路径并没有比之前找到的路径更短
//                         // 换句话说，我们找到了一个到达 sub_str 的路径，但它并不是最优的（即步数不够少）
//                         // cur[&poll] + 1 是 从 poll 变化 到 sub_str 的次数
//                         // cur[&sub_str] 是已经记录的，变化到 sub_str 的次数
//                         if cur.contains_key(&sub_str) && cur[&sub_str] <= cur[&poll] + 1 {
//                             continue;
//                         }

//                         if other.contains_key(&sub_str) {
//                             // 如果该字符串在「另一方向」出现过，说明找到了联通两个方向的最短路
//                             return cur[&poll] + 1 + other[&sub_str];
//                         } else {
//                             deque.push_back(sub_str.clone());
//                             cur.insert(sub_str, cur[&poll] + 1);
//                         }
//                     }
//                 }
//             }
//         }

//         -1i32
//     }
// }

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        use std::collections::VecDeque;

        let n = word_list.len();
        if n == 0 { return 0; }

        let target_i = match word_list.iter().position(|w| w == &end_word) {
            Some(i) => i,
            None => return 0,
        };

        fn diff_one(a: &[u8], b: &[u8]) -> bool {
            let mut diff = 0;
            for k in 0..a.len() {
                if a[k] != b[k] {
                    diff += 1;
                    if diff > 1 { return false; }
                }
            }
            diff == 1
        }

        let mut g = vec![Vec::<usize>::new(); n + 1]; // 0..n-1 words, n = begin

        for i in 0..n {
            for j in i + 1..n {
                if diff_one(word_list[i].as_bytes(), word_list[j].as_bytes()) {
                    g[i].push(j);
                    g[j].push(i);
                }
            }
        }

        for i in 0..n {
            if diff_one(begin_word.as_bytes(), word_list[i].as_bytes()) {
                g[n].push(i);
            }
        }

        let mut q = VecDeque::new();
        let mut vis = vec![false; n + 1];
        q.push_back((n, 1));
        vis[n] = true;

        while let Some((u, dist)) = q.pop_front() {
            for &v in &g[u] {
                if v == target_i { return dist + 1; }
                if !vis[v] {
                    vis[v] = true;
                    q.push_back((v, dist + 1));
                }
            }
        }

        0
    }
}


fn main() {
    println!("Hello, world!");
}
