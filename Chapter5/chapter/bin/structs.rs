struct S<'a> {
    r: &'a i32,
}

#[derive(Copy, Clone)]
struct Foo<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn sum_r_foo(r: &i32, foo: Foo) -> i32 {
    r + foo.x + foo.y
}

fn sum_r_foo_explicit<'a, 'b, 'c>(r: &'a i32, foo: Foo<'b, 'c>) -> i32 {
    r + foo.x + foo.y
}

fn main() {
    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }

    {
        let x = 10;
        let r;
        {
            let y = 20;
            {
                let foo = Foo { x: &x, y: &y };
                r = foo.x;
            }
        }
        println!("{}", r);
    }

    let foo = Foo { x: &10, y: &20 };
    assert_eq!(sum_r_foo(&70, foo), 100);
    assert_eq!(sum_r_foo_explicit(&70, foo), 100);
}
