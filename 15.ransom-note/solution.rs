use std::collections::hash_map;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = [0u32; 256];
        for c in magazine.bytes() {
            count[c as usize] += 1;
        }
        for c in ransom_note.bytes() {
            let c_ref = &mut count[c as usize];
            if *c_ref == 0 {
                return false;
            }
            *c_ref = *c_ref - 1;
        }
        true
    }
}
