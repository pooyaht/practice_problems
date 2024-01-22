pub struct Solution;
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return 1;
        }
        let mut ans = 1;
        let mut curr = 1;
        let mut k = 0;
        let mut direction = arr[0] < arr[1];
        while k < arr.len() - 1 {
            if arr[k] == arr[k + 1] {
                k += 1;
                curr = 1;
                continue;
            }
            if ((arr[k] < arr[k + 1]) && direction) || (arr[k] > arr[k + 1] && !direction) {
                direction = !direction;
                curr += 1;
                k += 1;
            } else {
                curr = 1;
                direction = arr[k] < arr[k + 1];
            }
            ans = ans.max(curr);
        }
        ans
    }
}
