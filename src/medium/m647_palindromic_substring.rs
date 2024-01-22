struct Solution;
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        fn extend_palindrome(s: &str, mut left: i32, mut right: i32) -> i32 {
            let s = s.chars().collect::<Vec<char>>();
            let mut count = 0;
            while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
                count += 1;
                left -= 1;
                right += 1;
            }
            count
        }
        let mut count = 0;
        for i in 0..s.len() {
            count += extend_palindrome(s.as_ref(), i as i32, i as i32);
            count += extend_palindrome(s.as_ref(), i as i32, i as i32 + 1);
        }

        count
    }
}
