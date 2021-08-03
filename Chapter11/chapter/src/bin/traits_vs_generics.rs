use std::fmt::Debug;

trait Vegetable: Debug {}

#[derive(Debug)]
struct Brinjal;
impl Vegetable for Brinjal {}

#[derive(Debug)]
struct Carrot;
impl Vegetable for Carrot {}

#[derive(Debug)]
struct Beetroot;
impl Vegetable for Beetroot {}

//struct Salad<V: Vegetable> {
//    veggies: Vec<V>,
//}

struct Salad {
    veggies: Vec<Box<dyn Vegetable>>,
}

fn main() {
    let salad = Salad {
        veggies: vec![Box::new(Brinjal), Box::new(Carrot), Box::new(Beetroot)],
    };

    for veggie in &salad.veggies {
        println!("{:?}", veggie);
    }
}
