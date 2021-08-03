#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MyOrdering {
    Less,
    Equal,
    Greater,
}

fn compare(n: i32, m: i32) -> MyOrdering {
    if n < m {
        MyOrdering::Less
    } else if n == m {
        MyOrdering::Equal
    } else {
        MyOrdering::Greater
    }
}

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

fn main() {
    assert_eq!(compare(1, 2), MyOrdering::Less);
    assert_eq!(compare(11, 2), MyOrdering::Greater);
    assert_eq!(compare(2, 2), MyOrdering::Equal);

    assert_eq!(std::mem::size_of::<MyOrdering>(), 1);
    assert_eq!(MyOrdering::Less as i32, 0);
    assert_eq!(MyOrdering::Equal as i32, 1);
    assert_eq!(MyOrdering::Greater as i32, 2);

    assert_eq!(TimeUnit::Months.plural(), "months");
    assert_eq!(TimeUnit::Days.singular(), "day");
}