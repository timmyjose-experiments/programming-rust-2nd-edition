struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    pub fn push(&mut self, elem: T) {
        self.younger.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<T>, Vec<T>) {
        (self.older, self.younger)
    }
}

fn main() {
    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };

    for num in 1..=10 {
        q.push(num);
    }

    while !q.is_empty() {
        println!("{}", q.pop().unwrap());
    }

    for num in 11..20 {
        q.push(num);
    }

    let _ = q.pop();
    let _ = q.pop();
    q.push(100);
    q.push(200);

    let (older, younger) = q.split();
    assert_eq!(older, vec![19, 18, 17, 16, 15, 14, 13]);
    assert_eq!(younger, vec![100, 200]);
}
