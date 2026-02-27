use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result = Vec::with_capacity(strs.len());
    let mut string_map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for word in strs {
        let mut string_k: Vec<char> = word.chars().collect();
        string_k.sort();
        string_map.entry(string_k).or_insert(vec![]).push(word);
    } 
    for (_, v) in string_map {
        result.push(v);
    }
    result
}

fn main() {
    let test_str = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
    println!("{:?}", group_anagrams(test_str));
}
