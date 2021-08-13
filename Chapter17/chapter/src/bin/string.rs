fn main() {
    let spaced = "man hat tan";
    let spaceless = spaced
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();
    assert_eq!(spaceless, "manhattan");

    let full = "bookkeeping";
    assert_eq!(&full[..], "bookkeeping");
    assert_eq!(&full[..4], "book");
    assert_eq!(&full[4..], "keeping");
    assert!(full[2..8].contains("ok"));
    assert_eq!(full[..].len(), full.len());

    let parenthesized = "Rust (錆)";
    assert_eq!(parenthesized[6..].chars().next(), Some('錆'));

    let mut also_spaceless = "con".to_string();
    also_spaceless.extend("tri but ion".split_whitespace());
    assert_eq!(also_spaceless, "contribution");

    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    let mut winston = "Churchill".to_string();
    winston.drain(2..6);
    assert_eq!(winston, "Chill");
}