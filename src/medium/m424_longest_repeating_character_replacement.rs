struct Solution;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut occurrences = [0; 26];
        let chars = s.chars().collect::<Vec<_>>();
        chars
            .iter()
            .fold(
                (0, 0, 0),
                |(mut window_left, window_right, mut max_substr_len), c| {
                    let right_char_index = *c as usize - 'A' as usize;
                    let left_char_index = chars[window_left] as usize - 'A' as usize;
                    occurrences[right_char_index] += 1;
                    let window_size = window_right - window_left + 1;
                    if window_size - occurrences.iter().max().unwrap_or(&0) > k.try_into().unwrap()
                    {
                        window_left += 1;
                        occurrences[left_char_index] -= 1;
                    } else {
                        max_substr_len = max_substr_len.max(window_size);
                    }
                    (window_left, window_right + 1, max_substr_len)
                },
            )
            .2
            .try_into()
            .unwrap()
    }
}
