impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let n = n as usize + 1;
        let mut dist = [0; 26];

        tasks
            .iter()
            .for_each(|&t| dist[(t as u8 - b'A') as usize] += 1);

        let max = *dist.iter().max().unwrap();
        let max_cnt = dist.iter().filter(|&&v| v == max).count();

        tasks.len().max((max - 1) * n + max_cnt) as i32
    }
}
