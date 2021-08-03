use std::rc::Rc;

struct Queue<T> {
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
}

struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}

impl Node {
    pub fn new(tag: &str) -> Self {
        Node {
            tag: tag.to_string(),
            children: Vec::new(),
        }
    }

    //pub fn append_to(self, parent: &mut Node) {
    //    parent.children.push(Rc::new(self));
    //}
    pub fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

fn main() {
    let mut bq = Box::new(Queue::new());
    bq.push('x');
    assert_eq!(bq.pop(), Some('x'));

    let mut parent = Node::new("initial");
    let shared_node = Rc::new(Node::new("first"));
    shared_node.append_to(&mut parent);
}