// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut low = 0;
        let mut high = n;
        let mut mid;
        while low < high {
            mid = low + (high - low) / 2;
            if self.isBadVersion(mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low
    }
}
