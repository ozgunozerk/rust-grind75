impl Solution {
    pub fn ladder_length(
        begin_word: String,
        end_word: String,
        mut word_list: Vec<String>,
    ) -> i32 {
        let end_id =
            if let Some((id, _)) = word_list.iter().enumerate().find(|(_, e)| *e == &end_word) {
                id
            } else {
                return 0;
            };
        let (word_list, begin_id) = if let Some((id, _)) = word_list
            .iter()
            .enumerate()
            .find(|(id, e)| *e == &begin_word)
        {
            (word_list, id)
        } else {
            word_list.push(begin_word.clone());
            let len = word_list.len();
            (word_list, len - 1)
        };
        let mut word_list: std::collections::HashSet<_> = word_list
            .into_iter()
            .map(|e| e.as_bytes().to_vec())
            .collect();
        let (mut process_l, mut process_r) = (Vec::new(), Vec::new());
        let len = begin_word.as_bytes().len();
        let l_item = begin_word.as_bytes().to_vec();
        let r_item = end_word.as_bytes().to_vec();
        word_list.remove(&l_item);
        word_list.remove(&r_item);
        let mut step = 1;
        process_l.push(l_item);
        process_r.push(r_item);
        while !process_l.is_empty() && !process_r.is_empty() {
            let (mut this, mut other, mut this_process) = if process_l.len() >= process_r.len() {
                let drained: Vec<_> = process_r.drain(..).collect();
                (drained, &process_l, &mut process_r)
            } else {
                let drained: Vec<_> = process_l.drain(..).collect();
                (drained, &process_r, &mut process_l)
            };
            for mut this_word in this {
                let mut other_iter = other.iter();
                while let Some(other_word) = other_iter.next() {
                    if Self::_can_jump(other_word, &this_word) {
                        return step + 1;
                    }
                }
                for i in 0..len {
                    let temp = this_word[i];
                    for this_char in 'a' as u8..='z' as u8 {
                        this_word[i] = this_char;
                        if word_list.contains(&this_word) {
                            word_list.remove(&this_word);
                            this_process.push(this_word.clone());
                        }
                    }
                    this_word[i] = temp;
                }
            }
            step += 1;
        }
        0
    }
    fn _can_jump(l: &[u8], r: &[u8]) -> bool {
        l.iter()
            .zip(r.iter())
            .fold(0, |acc, (l, r)| acc + (l != r) as i32)
            == 1
    }
}
