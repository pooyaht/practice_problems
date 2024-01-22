struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if let Some('(') = stack.pop() {
                        continue;
                    }
                    return false;
                }
                ']' => {
                    if let Some('[') = stack.pop() {
                        continue;
                    }
                    return false;
                }
                '}' => {
                    if let Some('{') = stack.pop() {
                        continue;
                    }
                    return false;
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}
