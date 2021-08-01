fn main() {
    println!(
        r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
        "###
    );

    // byte strings
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}
