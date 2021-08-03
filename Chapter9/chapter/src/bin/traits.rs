#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let pt = Point {
        x: 21.033,
        y: -233.11,
    };

    println!("{:?}", pt);

    let another_pt = pt;
    assert_eq!(pt, another_pt);
}
