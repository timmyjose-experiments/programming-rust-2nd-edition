fn main() {
    reference_demo();
    box_demo();
    raw_pointers();
}

fn reference_demo() {
    {
        let x = 42;
        let ref_x = &x;
        assert_eq!(x, *ref_x);

        let another_ref_x = &x;
        assert_eq!(x, *another_ref_x);
    }

    {
        let mut x = 42;
        let ref_x = &mut x;
        assert_eq!(42, *ref_x);

        let another_ref_x = &mut x;
        assert_eq!(42, *another_ref_x);
    }
}

fn box_demo() {
    let t = Box::new((12, "eggs"));
    assert_eq!(*t, (12, "eggs"));
}

fn raw_pointers() {
    let x = 42;
    let raw_x = &x as *const i32 as *mut i32;

    unsafe {
        *raw_x += 100;
        assert_eq!(*raw_x, 142);
    }
}
