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

pub enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { center: Point3d, corner2: Point3d },
}

pub struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3d {
    const ORIGIN: Point3d = Point3d {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

fn main() {
    println!(
        "sizeof TimeUnit = {}, sizeof RoughTime = {}",
        std::mem::size_of::<TimeUnit>(),
        std::mem::size_of::<RoughTime>()
    );
    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    println!("{:?}", four_score_and_seven_years_ago);

    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    println!("{:?}", three_hours_from_now);

    println!("sizeof Shape = {}", std::mem::size_of::<Shape>());

    let unit_sphere = Shape::Sphere {
        center: Point3d::ORIGIN,
        radius: 1.0,
    };
}
