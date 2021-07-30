fn main() {
    print_padovan();

    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point);
        assert_eq!(label, "(0.625, 0.5)");
    } // dropped here

    {
        struct Person {
            name: String,
            birth: i32,
        }

        let mut composers = Vec::new();

        composers.push(Person {
            name: "Palestrina".to_string(),
            birth: 1525,
        });

        composers.push(Person {
            name: "Dowland".to_string(),
            birth: 1563,
        });

        composers.push(Person {
            name: "Lully".to_string(),
            birth: 1632,
        });

        for composer in &composers {
            println!("{} born {}", composer.name, composer.birth);
        }
    } // dropped here

    {
        use std::rc::Rc;

        let s: Rc<String> = Rc::new("shirataki".to_string());
        let t = s.clone();
        let u = s.clone();
        println!("{}", Rc::strong_count(&s));
        println!("{}", Rc::weak_count(&s));

        assert!(s.contains("shira"));
        assert_eq!(u.len(), 9);
        assert_eq!(t, u);
    }
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 1] + padovan[i - 2];
        padovan.push(next);
    }

    println!("P(1..10) = {:?}", padovan);
} // dropped here
