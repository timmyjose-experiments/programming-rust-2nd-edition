use std::ops::Drop;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        let person = Person { name, age };
        println!("Created {:?}", person);
        person
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    println!("Welcome to main");
    {
        let bob = Person::new("Bob".to_string(), 42);
        {
            let angus = Person::new("Angus".to_string(), 33);
            std::mem::drop(angus);
            let sarah = Person::new("Sarah".to_string(), 21);
        }
    }
    println!("Adios from main");
}