impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut arr = [1, 2];
        match n {
            1 | 2 => arr[(n - 1) as usize],
            _ => {
                for i in 2..n {
                    arr[(i & 1) as usize] = arr[0] + arr[1];
                }
                arr[((n - 1) & 1) as usize]
            }
        }
    }
}
