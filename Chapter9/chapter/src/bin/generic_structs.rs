pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, e: T) {
        self.younger.push(e);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            std::mem::swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

fn main() {
    let mut q = Queue::new();

    for num in 1..=10 {
        q.push(num);
    }

    while !q.is_empty() {
        println!("{}", q.pop().unwrap());
    }
}
