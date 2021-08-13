fn main() {
    let haystack = "one fine day, in the middle of the night";

    assert_eq!(haystack.find(','), Some(12));
    assert_eq!(haystack.find("night"), Some(35));
    assert_eq!(haystack.find(char::is_whitespace), Some(3));

    let code = "\t      function noodle() {}";
    assert_eq!(
        code.trim_start_matches([' ', '\t'].as_ref()),
        "function noodle() {}"
    );

    let good_utf8 = vec![0xe9, 0x8c, 0x86];
    assert_eq!(String::from_utf8(good_utf8).ok(), Some('錆'.to_string()));

    let bad_utf8 = vec![0x9f, 0xf0, 0xa6, 0x80];
    let result = String::from_utf8(bad_utf8);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().into_bytes(),
        vec![0x9f, 0xf0, 0xa6, 0x80]
    );
}
