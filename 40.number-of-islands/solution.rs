impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut output = 0;
        let (len_outer, len_inner) = (grid.len(), grid[0].len());
        let mut cache = Vec::<(usize, usize)>::with_capacity(len_inner * len_outer);
        for i in 0..len_outer {
            for j in 0..len_inner {
                if grid[i][j] == '1' {
                    cache.push((i, j));
                    output += 1;
                    while let Some((i, j)) = cache.pop() {
                        grid[i][j] = '0';
                        if i + 1 < len_outer && grid[i + 1][j] == '1' {
                            cache.push((i + 1, j));
                        }
                        if i > 0 && grid[i - 1][j] == '1' {
                            cache.push((i - 1, j));
                        }
                        if j + 1 < len_inner && grid[i][j + 1] == '1' {
                            cache.push((i, j + 1));
                        }
                        if j > 0 && grid[i][j - 1] == '1' {
                            cache.push((i, j - 1));
                        }
                    }
                }
            }
        }
        output
    }
}
