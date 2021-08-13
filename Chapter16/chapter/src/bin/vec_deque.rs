// VecDeque

use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::from(('a'..='z').collect::<Vec<_>>());

    while !deque.is_empty() {
        if let Some(front) = deque.pop_front() {
            println!("front = {}", front);
        }

        if let Some(back) = deque.pop_back() {
            println!("back = {}", back);
        }
    }
}