impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let len = nums.len();
        let mut result = Vec::with_capacity(1000);

        for idx in 0..len {
            if idx != 0 && nums[idx] == nums[idx - 1] {
                continue;
            }
            let mut left = idx + 1;
            let mut right = len - 1;
            while left < right {
                match nums[idx] + nums[left] + nums[right] {
                    x if x < 0 => left = left + 1,
                    x if x > 0 => right = right - 1,
                    x if x == 0 => {
                        result.push(vec![nums[idx], nums[left], nums[right]]);
                        left = left + 1;
                        while left < right && nums[left] == nums[left - 1] {
                            left += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        result
    }
}
