use std::collections::{BTreeSet, HashSet};

fn main() {
    let mut h1 = BTreeSet::new();
    for num in 1..=10 {
        h1.insert(num);
    }

    for num in (1..=10).rev() {
        h1.insert(num);
    }

    for elem in &h1 {
        println!("{}", elem);
    }
    println!();

    let mut h2 = BTreeSet::new();
    for num in (1..=10).rev() {
        h2.insert(num);
    }

    for elem in &h2 {
        println!("{}", elem);
    }

    assert!(h2.contains(&1));
    assert!(!h2.contains(&11));

    h2.remove(&1);
    h2.remove(&3);
    h2.remove(&2);
    h2.remove(&5);
    h2.insert(21);
    h2.insert(19);
    h2.insert(32);

    println!("h1 = {:?}", h1);
    println!("h1 = {:?}", h2);

    let intersection = &h1 & &h2;
    println!("{:?}", intersection);

    let union = &h1 | &h2;
    println!("{:?}", union);

    let difference1 = &h1 - &h2;
    println!("{:?}", difference1);

    let difference2 = &h2 - &h1;
    println!("{:?}", difference2);

    let symm_diff = &h1 ^ &h2;
    println!("{:?}", symm_diff);
}