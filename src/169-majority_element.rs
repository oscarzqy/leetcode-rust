impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut majority = nums[0];
        let mut count = 1;
        for val in nums.iter().skip(1) {
            match *val == majority {
                true => {
                    count += 1;
                },
                false => {
                    count -= 1;
                    if (count == 0) {
                        majority = *val;
                        count = 1;
                    }
                }
            }
        }

        return majority as i32;
    }
}