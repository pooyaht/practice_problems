struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s = s;
        s.make_ascii_lowercase();
        s.retain(|c| c.is_alphanumeric());
        s.trim().chars().rev().eq(s.trim().chars())
    }
}
