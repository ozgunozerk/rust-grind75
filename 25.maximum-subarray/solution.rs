impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = i32::MIN;
        let mut temp = 0;

        for num in nums.into_iter() {
            temp = num.max(temp + num);
            sum = sum.max(temp);
        }
        sum
    }
}
