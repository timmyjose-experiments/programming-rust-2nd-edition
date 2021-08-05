use std::default::Default;

struct Person {
    name: String,
    age: i32,
}

impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::new(),
            age: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct Foo {
    name: String,
    age: i32,
}

fn main() {
    let person = Person::default();
    assert_eq!(person.name, String::new());
    assert_eq!(person.age, 0);

    let foo = Foo::default();
    let bar = Foo::default();
    assert_eq!(foo, bar);
}