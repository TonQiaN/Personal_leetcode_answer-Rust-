struct Solution {}
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut result = vec![];
        let mut anagrams_map = HashMap::new();
        for single_str in strs {
            let mut word_sort = single_str.chars().collect::<Vec<_>>();
            word_sort.sort();
            anagrams_map
                .entry(word_sort)
                .and_modify(|inner_vec: &mut Vec<String>| inner_vec.push(single_str.clone()))
                .or_insert(vec![single_str]);
        }
        for (_, v) in anagrams_map {
            result.push(v);
        }
        result
    }
}

// use std::collections::HashMap;

// pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
//     let mut result = Vec::with_capacity(strs.len());
//     let mut string_map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
//     for word in strs {
//         let mut string_k: Vec<char> = word.chars().collect();
//         string_k.sort();
//         string_map.entry(string_k).or_insert(vec![]).push(word);
//     }
//     for (_, v) in string_map {
//         result.push(v);
//     }
//     result
// }

fn main() {
}
