use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hashmap = HashMap::with_capacity(s.len() + t.len());
        s.chars().for_each(|c| {
            hashmap.insert(c, hashmap.get(&c).unwrap_or(&0) + 1);
        });
        t.chars().for_each(|c| {
            hashmap.insert(c, hashmap.get(&c).unwrap_or(&0) - 1);
        });
        hashmap.values().all(|&v| v == 0)
    }
}
