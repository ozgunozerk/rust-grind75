struct Entry {
    val: i32,
    min: i32,
}

#[derive(Default)]
struct MinStack {
    stack: Vec<Entry>,
}

impl MinStack {
    fn new() -> Self {
        Self::default()
    }

    fn push(&mut self, val: i32) {
        let min = self
            .stack
            .last()
            .map_or(val, |last| if last.min > val { val } else { last.min });
        self.stack.push(Entry { val, min });
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().val
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().min
    }
}
