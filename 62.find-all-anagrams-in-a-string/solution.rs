impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s_len, p_len) = (s.len(), p.len());

        if p_len > s_len {
            return vec![];
        }

        let (mut c, mut res) = ([0; 26], vec![]);
        p.chars().for_each(|ch| c[(ch as u8 - b'a') as usize] += 1);

        let (first, rest) = s.split_at(p_len);
        for ch in first.chars() {
            c[(ch as u8 - b'a') as usize] -= 1;
        }

        if c == [0; 26] {
            res.push(0);
        }

        s.chars()
            .zip(rest.chars())
            .enumerate()
            .for_each(|(idx, (s_ch, rest_ch))| {
                c[(s_ch as u8 - b'a') as usize] += 1;
                c[(rest_ch as u8 - b'a') as usize] -= 1;
                if c == [0; 26] {
                    res.push(idx as i32 + 1);
                }
            });

        res
    }
}
