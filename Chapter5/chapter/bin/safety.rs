static mut STASH: &'static i32 = &0;

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn g<'a>(_p: &'a i32) {}

fn smallest(slice: &[i32]) -> &i32 {
    let mut min = &slice[0];
    for elem in &slice[1..] {
        if *elem <= *min {
            min = elem;
        }
    }

    min
}

fn main() {
    //{
    //    let r;
    //    {
    //        let x = 1;
    //        r = &x;
    //    }
    //    assert_eq!(*r, 1);
    //}

    {
        let x = 1;
        {
            let r = &x;
            assert_eq!(*r, 1);
        }
    }

    f(&42);

    let y = 100;
    g(&y);

    let nums = [1, 2, 3, -1, 10, 0];
    assert_eq!(*smallest(&nums), -1);
}
