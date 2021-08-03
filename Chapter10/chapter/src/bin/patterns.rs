#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    pub fn plural(&self) -> &'static str {
        match *self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    pub fn singular(&self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, 1) => format!("1 {} ago", units.singular()),
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, 1) => format!("1 {} from now", units.singular()),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}

fn greet_people(people: &[&str]) {
    match people {
        [] => println!("All alone"),
        [a] => println!("Hello, {}", a),
        [a, b] => println!("Welcome {} and {}", a, b),
        [a, .., b] => println!("Welcome to all from {} to {}", a, b),
    }
}

fn main() {
    println!(
        "{}",
        rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Months, 1))
    );
    println!(
        "{}",
        rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Months, 3))
    );

    greet_people(&[]);
    greet_people(&["Tom"]);
    greet_people(&["Jerry"]);
    greet_people(&["Tom", "Dick", "Harry", "Jerry"]);
}
