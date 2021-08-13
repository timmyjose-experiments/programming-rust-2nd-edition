use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);

    for num in 1..=10 {
        heap.push(num);
    }

    assert_eq!(heap.peek(), Some(&10));
    assert_eq!(heap.len(), 10);

    while !heap.is_empty() {
        println!("Popping {:?}", heap.pop().unwrap());
    }

    min_heap_demo();
}

fn min_heap_demo() {
    use std::cmp::Reverse;

    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);

    for num in 1..=10 {
        heap.push(Reverse(num));
    }

    assert_eq!(heap.peek(), Some(&Reverse(1)));

    while !heap.is_empty() {
        println!("Popping {:?}", heap.pop().unwrap().0);
    }
}
