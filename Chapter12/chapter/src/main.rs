use std::ops::Add;

//trait Add<Rhs=Self> {
//    type Output;
//
//    fn add(self, rhs: Rhs) -> Self::Output;
//}

fn main() {
    println!("Operator overloading");

    println!("{}", 1 + 2);
    println!("{}", 1.add(2));
}
