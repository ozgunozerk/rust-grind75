use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default)]
/// we will divide the list into 2 parts: lower and higher
/// for the lower part, we will use a MaxHeap
/// for the higher part, we will use a MinHeap
/// based on the length, we can return the Max value in lower part,
/// or Min value in high part, or take their average
/// Insertion: O(logn)
/// find: O(1)
struct MedianFinder {
    lo: BinaryHeap<i32>,
    hi: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, num: i32) {
        // add the element to lower part
        self.lo.push(num);
        // add the biggest num in lower part ot high part
        self.hi.push(Reverse(*self.lo.peek().unwrap()));
        // remove the biggest num in lower part
        self.lo.pop();
        if self.lo.len() < self.hi.len() {
            // balance the lower and higher part
            self.lo.push(self.hi.peek().unwrap().0);
            self.hi.pop();
        }
    }

    fn find_median(&self) -> f64 {
        if self.lo.len() > self.hi.len() {
            return *self.lo.peek().unwrap() as f64;
        }
        (self.lo.peek().unwrap() + self.hi.peek().unwrap().0) as f64 / 2.0
    }
}
