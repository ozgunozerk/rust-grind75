impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ans = vec![];

        Self::solve(nums, 0, n, &mut ans);

        ans
    }

    fn solve(mut nums: Vec<i32>, l: usize, r: usize, ans: &mut Vec<Vec<i32>>) {
        if l == r { return ans.push(nums) }
        for i in l..r {
            nums.swap(l, i);
            Self::solve(nums.clone(), l + 1, r, ans);
        }
    }
}
