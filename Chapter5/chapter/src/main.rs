use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("Works by {}", artist);
        for work in works {
            println!("\t{}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort();
    }
}

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    sort_works(&mut table);
    println!();
    show(&table);

    // implicit dereference by the . operator

    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };

    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert!(anime_ref.bechdel_pass);
    assert_eq!((*anime_ref).name, "Aria: The Animation");
    assert!((*anime_ref).bechdel_pass);

    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 100, y: 200 };
    let r = &point;
    let rr = &r;
    let rrr = &rr;
    assert_eq!(rrr.x, 100);
    assert_eq!(rrr.y, 200);

    // implicit borrowing by the . operator

    let mut nums = vec![1, 2, 5, 3, 4];
    nums.sort();
    assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    (&mut nums).sort_by(|x, y| y.cmp(x));
    assert_eq!(nums, vec![5, 4, 3, 2, 1]);

    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    assert_eq!(&factorial(6) + &1000, 1720);
}
