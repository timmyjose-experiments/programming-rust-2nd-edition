// vectors

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let num_slice = &numbers[3..];
    assert_eq!(num_slice.len(), 2);
    assert_eq!(num_slice, [4, 5]);

    let copy_of_numbers = numbers.iter().collect::<Vec<_>>();
    let another_copy = copy_of_numbers[..].to_vec();
    assert_eq!(another_copy.len(), numbers.len());

    if let Some(&first) = numbers.first() {
        println!("First element of numbers is {}", first);
    }

    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "again".to_string(),
        "we".to_string(),
        "meet".to_string(),
    ];

    if let Some(last) = words.last() {
        println!("Last word in words = {}", last);
    }

    assert_eq!(words[..].get(2), Some(&"again".to_string()));

    let mut slice = [0, 1, 2, 3];
    {
        let last = slice.last_mut().unwrap();
        assert_eq!(*last, 3);
        *last = 100;
    }
    assert_eq!(slice, [0, 1, 2, 100]);

    move_demo();
    borrow_demo();
    borrow_mut_demo();

    let reversed_hello = reverse_string("hello".to_string());
    println!("{}", reversed_hello);

    misc_demo();
}

fn misc_demo() {
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let arr = matrix.concat();
    println!("concatenated matrix = {:?}", arr);

    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let arr = matrix.join(&0);
    println!("joined matrix = {:?}", arr);
}

fn move_demo() {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "again".to_string(),
        "we".to_string(),
        "meet".to_string(),
    ];

    for word in words {
        println!("{}", word);
    }

    //println!("{:?}", words);
}

fn borrow_demo() {
    let words = vec![
        "hello".to_string(),
        "world".to_string(),
        "again".to_string(),
        "we".to_string(),
        "meet".to_string(),
    ];

    for word in &words {
        println!("{}", *word);
    }

    println!("{:?}", words);
}

fn borrow_mut_demo() {
    let mut words = vec![
        "hello".to_string(),
        "world".to_string(),
        "again".to_string(),
        "we".to_string(),
        "meet".to_string(),
    ];

    for word in &mut words {
        word.push('!');
    }

    println!("{:?}", words);
    println!("{}", std::mem::size_of_val(&words));

    while !words.is_empty() {
        let _ = words.pop();
    }
    println!("{}", std::mem::size_of_val(&words));
    assert!(words.is_empty());
    words.shrink_to_fit();
    println!("len = {}, capacity = {}", words.len(), words.capacity());
    println!("{}", std::mem::size_of_val(&words));
}

fn reverse_string(word: String) -> String {
    let mut buf = word.into_bytes();
    let mut reversed = String::new();

    while !buf.is_empty() {
        reversed.push(buf.pop().unwrap() as char);
    }

    reversed
}