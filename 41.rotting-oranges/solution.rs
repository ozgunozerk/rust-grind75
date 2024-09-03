use std::collections::HashSet;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut rotten = vec![];
        let mut fresh = HashSet::new();
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    rotten.push((i, j));
                } else if grid[i][j] == 1 {
                    fresh.insert((i, j));
                }
            }
        }
        if !fresh.is_empty() && rotten.is_empty() {
            return -1;
        }
        let mut result = 0;
        while !fresh.is_empty() {
            result += 1;
            let mut new_rotten = vec![];
            while let Some((i, j)) = rotten.pop() {
                // up
                if i > 0 && fresh.contains(&(i - 1, j)) {
                    fresh.remove(&(i - 1, j));
                    new_rotten.push((i - 1, j));
                }
                // right
                if j < n - 1 && fresh.contains(&(i, j + 1)) {
                    fresh.remove(&(i, j + 1));
                    new_rotten.push((i, j + 1));
                }
                // down
                if i < m - 1 && fresh.contains(&(i + 1, j)) {
                    fresh.remove(&(i + 1, j));
                    new_rotten.push((i + 1, j));
                }
                // left
                if j > 0 && fresh.contains(&(i, j - 1)) {
                    fresh.remove(&(i, j - 1));
                    new_rotten.push((i, j - 1));
                }
            }
            // we are not making progress so there is no way to continue
            if new_rotten.is_empty() {
                return -1;
            }
            rotten = new_rotten;
        }
        result
    }
}
