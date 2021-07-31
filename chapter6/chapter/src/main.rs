fn main() {
    let strings = vec!["Hello".to_string(), "world".to_string()];

    for s in strings {
        println!("{}", s);
    }

    //println!("{:?}", strings);

    let strings = vec!["Hello".to_string(), "world".to_string()];
    for s in &strings {
        println!("{}", s);
    }
    println!("{:?}", strings);

    let mut more_strings = vec!["Hola".to_string(), "mundo".to_string()];
    for s in &mut more_strings {
        s.push('!');
    }
    println!("{:?}", more_strings);

    let mut num = 1;
    let answer = loop {
        if num % 2 == 0 {
            break num;
        }
        num += 1;
    };
    assert_eq!(answer, 2);

    num = 1;
    let answer = loop {
        if num % 2 != 0 {
            num += 1;
            continue;
        }
        break num;
    };
    assert_eq!(answer, 2);
}
