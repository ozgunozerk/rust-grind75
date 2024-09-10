impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        // to be able to split the list into 2 equal part
        // the target should be the half of the sum
        let target = (sum / 2) as usize;

        // dp array represents if we can reach `target` sum
        let mut dp = vec![false; target + 1];

        // dp[target] = true, since at index `target`, we already reached the sum target
        // our aim is to make `dp[0] = true`, this means we can create a subset sum that is equal to target
        // example: `dp[5] = true` means, we can reach to the `target` if we somehow can reach `5` first
        dp[target] = true;

        for num in nums {
            for i in 0..=(target as i32 - num) {
                // mark true every index in `dp` if we can add current number and reach to `true`
                dp[i as usize] |= dp[(i + num) as usize];
            }
            // if we found a solution, terminate early
            if dp[0] == true {
                return true;
            }
        }

        dp[0]
    }
}
