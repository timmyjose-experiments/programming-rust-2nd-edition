use std::fmt::Display;

//use std::iter;
//use std::vec::IntoIter;

//fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
//    v.into_iter().chain(u.into_iter()).cycle()
//}

//fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
//    Box::new(v.into_iter().chain(u.into_iter()).cycle())
//}

fn cyclical_zip(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn print<T: Display>(val: T) {
    println!("{}", val);
}

fn impl_print(val: impl Display) {
    println!("{}", val);
}

fn main() {
    let v = cyclical_zip(vec![1, 2, 3, 4, 5], vec![b'a', b'b', b'c', b'd', b'e'])
        .take(11)
        .collect::<Vec<_>>();
    println!("{:?}", v);

    print("hello".to_string());
    print::<i32>(42);
    impl_print("world".to_string());
    //impl_print::<i32>(42);
}
