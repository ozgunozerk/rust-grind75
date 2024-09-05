use std::collections::HashMap;

#[derive(Default)]
struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        self.map.get(&key).and_then(|vec| match vec.partition_point(|&(t, _)| t <= timestamp) {
            0 => None,
            parition_point => Some(vec[parition_point - 1].1.clone()),
            // partition_point() returns the index of the element that is equal to the timestamp
            // hence, we substract 1 from the given index to find the largest element
            // this will work assuming that each new entry will have increasing timestamps
        }).unwrap_or_default()
    }
}
