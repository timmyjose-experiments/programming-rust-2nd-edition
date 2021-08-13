use std::borrow::Cow;
use std::path::Path;

fn main() {
    println!("{}", get_name());
    println!("{}", get_cow());
    text_formatting_demo();

    let path = Path::new("/Users/foo/bar/baz");
    println!("{}", path.display());
}

fn get_name() -> String {
    std::env::var("USER").unwrap_or("whoerver you are".to_string())
}

fn get_cow() -> Cow<'static, str> {
    std::env::var("USER")
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed("whoever you are"))
}

fn text_formatting_demo() {
    let text = "bookends";

    println!("{}", text);
    println!("{:4}", text);
    println!("{:.4}", text);
    println!("{:12.20}", text);
    println!("{:4.20}", text);
    println!("{:4.5}", text);
    println!("{:5.5}", text);
    println!("{:<12}", text);
    println!("{:^12}", text);
    println!("{:>12}", text);
    println!("{:=^12}", text);
    println!("{:*>12.5}", text);
}
