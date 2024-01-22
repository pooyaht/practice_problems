use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let hashmap = strs.into_iter().fold(
            HashMap::new(),
            |mut acc: HashMap<String, Vec<String>>, s| {
                let mut chars = s.chars().collect::<Vec<_>>();
                chars.sort();
                let key = chars.into_iter().collect::<String>();
                acc.entry(key).or_insert(vec![]).push(s);
                acc
            },
        );
        hashmap.into_iter().map(|(_, v)| v).collect()
    }
}
