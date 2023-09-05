impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n:usize = nums.len();
        let mut last_num = -1000;
        let mut matched = 0;
        for cur in 0..n {
            match nums[cur] == last_num {
                true => (),
                false => {
                    nums[matched] = nums[cur];
                    matched += 1;
                    last_num = nums[cur];
                }
            }
        }
        return matched as i32;
    }
}