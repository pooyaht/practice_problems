struct Solution;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut fast: i32 = 0;
        let mut slow: i32 = 0;
        loop {
            fast = nums[nums[fast as usize] as usize];
            slow = nums[slow as usize];
            if fast == slow {
                break;
            }
        }
        let mut head: i32 = 0;
        while head != slow {
            head = nums[head as usize];
            slow = nums[slow as usize];
        }
        head
    }
}
