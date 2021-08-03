#[derive(Debug)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    // associated constants (namespaced within the type)
    const NAME: &'static str = "Vector2";
    const ID: usize = 1;
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 1.0 };
}

fn main() {
    println!("{:?}, {:?}", Vector2::ZERO, Vector2::UNIT);
    assert_eq!(Vector2::NAME, "Vector2");
    assert_eq!(Vector2::ID, 1);
}