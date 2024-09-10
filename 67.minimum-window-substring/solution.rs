use std::collections::HashMap;

impl Solution
{
    pub fn min_window(s: String, t: String) -> String
    {
        let ss = s.as_bytes();

        // [1] obtain letter frequencies
        let mut freq: HashMap<u8,i32> = t
            .bytes()
            .fold(HashMap::new(), |mut f,ch| { *f.entry(ch).or_default() += 1; f });

        // [2] in this solution, we use a two-pointer sliding window
        //    approach while keeping track of a number of unique
        //    characters from 't' that are missing on each iteration
        let mut miss      = freq.len() as i32;
        let (mut l, mut r)  = (0, 0);
        let (mut wl, mut wr) = (0, s.len()+1);

        loop
        {
            // [3] move right pointer while we're still
            //    missing any of the characters from 't'
            if r < ss.len() && miss > 0
            {
                if let Some(f) = freq.get_mut(&ss[r])
                {
                    *f -= 1;
                    if *f == 0 { miss -= 1; }
                }
                r += 1;
            }

            // [4] move left pointer while we're still
            //    having a surplus of characters from 't'
            else if l < ss.len() && miss <= 0
            {
                // [5] the update of minimal substring happens only when
                //    we move the left pointer, i.e., when we already
                //    have no characters from 't' missing
                if r-l < wr-wl { wl = l; wr = r; }

                if let Some(f) = freq.get_mut(&ss[l])
                {
                    if *f == 0 { miss += 1; }
                    *f += 1;
                }
                l += 1;
            }
            else { break; }
        }

        return if wr <= ss.len() { s[wl..wr].to_string() } else { String::new() };
    }
}
