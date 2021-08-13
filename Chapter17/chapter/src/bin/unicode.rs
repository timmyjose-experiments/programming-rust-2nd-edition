fn latin1_to_char(c: u8) -> char {
    c as char
}

fn char_to_latin1(c: char) -> Option<u8> {
    if c as u32 <= 0xff {
        Some(c as u8)
    } else {
        None
    }
}

fn main() {
    assert_eq!(char_to_latin1(latin1_to_char(b'x')), Some(b'x'));

    let sabi = '\u{9306}';
    println!("{}", sabi);

    let crab = '\u{1f980}';
    println!("{}", crab);

    let arabic_word = "وَاو هَمْزة";
    for c in arabic_word.chars() {
        println!("{}", c);
    }
}