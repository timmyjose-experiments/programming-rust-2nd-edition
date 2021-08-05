use std::thread;

#[derive(Debug)]
struct City {
    name: String,
    population: i64,
    country: String,
}

impl City {
    fn new(name: String, population: i64, country: String) -> Self {
        City {
            name,
            population,
            country,
        }
    }
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| city.population);
}

fn sort_in_thread(mut cities: Vec<City>) -> thread::JoinHandle<Vec<City>> {
    let key_fn = |city: &City| -city.population;

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn count_cities_by_criterion(cities: &Vec<City>, check_fn: fn(&City) -> bool) -> usize {
    cities.iter().filter(|city| check_fn(city)).count()
}

fn count_cities_by_criterion_closure<F>(cities: &Vec<City>, check_fn: F) -> usize
where
    F: Fn(&City) -> bool,
{
    cities.iter().filter(|city| check_fn(city)).count()
}

fn main() {
    let mut cities = vec![
        City::new("Bangalore".to_string(), 10_000_000, "India".to_string()),
        City::new("Los Angeles".to_string(), 12_000_000, "USA".to_string()),
        City::new("Tokyo".to_string(), 20_000_000, "Japan".to_string()),
    ];

    sort_cities(&mut cities);
    println!("{:#?}", cities);

    let handle = sort_in_thread(cities);
    let sorted_cities = handle.join().unwrap();
    println!("Sorted cities: {:#?}", sorted_cities);

    let cities = sorted_cities;
    println!(
        "How many cities are above 10 million in terms of population? {}",
        count_cities_by_criterion(&cities, |city| city.population > 10_000_000)
    );
    println!(
        "How many cities are above 10 million in terms of population? {}",
        count_cities_by_criterion_closure(&cities, |city| city.population > 10_000_000)
    );
}