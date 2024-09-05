impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let nums_len = nums.len();
        if nums_len < 2 {
            return;
        }
        let mut zeros_inserted = 0;
        let mut ones_inserted = 0;
        let mut twos_inserted = 0;
        let mut i = 0;
        loop {
            match nums.remove(i) {
                0 => {
                    nums.insert(0, 0);
                    zeros_inserted += 1;
                    i += 1;
                }
                1 => {
                    nums.insert(zeros_inserted, 1);
                    if i <= zeros_inserted + ones_inserted {
                        i += 1
                    };
                    ones_inserted += 1;
                }
                2 => {
                    nums.push(2);
                    twos_inserted += 1;
                    if i >= (nums_len - twos_inserted) {
                        i += 1;
                    }
                }
                _ => unreachable!("you expected 0, 1, or 2... But it's me, DIO!"),
            }

            if i >= nums_len - twos_inserted {
                break;
            }
        }
    }
}
