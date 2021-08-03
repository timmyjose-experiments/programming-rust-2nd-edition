use std::iter::Iterator;

fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut res = Vec::new();
    for e in iter {
        res.push(e);
    }
    res
}

fn main() {
    let nums = collect_into_vector(vec![1, 2, 4, 5].into_iter());
    println!("{:?}", nums);
}
