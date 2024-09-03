impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut current: Vec<i32> = Vec::new();

        fn backtrack(
            result: &mut Vec<Vec<i32>>,
            candidates: &[i32],
            target: i32,
            current: &mut Vec<i32>,
        ) {
            if target == 0 {
                result.push(current.to_vec());
                return;
            }
            if target < 0 {
                return;
            }
            for (i, &candidate) in candidates.iter().enumerate() {
                current.push(candidate);
                backtrack(result, &candidates[i..], target - candidate, current);
                current.pop();
            }
        }

        backtrack(&mut result, &candidates, target, &mut current);
        result
    }
}
