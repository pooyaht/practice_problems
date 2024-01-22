struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hashset: std::collections::HashSet<char> = std::collections::HashSet::new();
        let chars = s.chars().collect::<Vec<_>>();
        chars
            .iter()
            .fold((0, 0, 0), |(mut left, right, mut max_sub_str), c| {
                while hashset.contains(c) {
                    hashset.remove(&chars[left]);
                    left += 1;
                }
                max_sub_str = max_sub_str.max(right - left + 1);
                hashset.insert(*c);
                (left, right + 1, max_sub_str)
            })
            .2 as i32
    }
}
