#[derive(Debug)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(broom: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: broom.height / 2,
        ..broom
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

fn main() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (0.0, 0.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (b1, b2) = chop(hokey);
    assert_eq!(b1.name, "Hokey I");
    assert_eq!(b1.height, 30);
    assert_eq!(b1.health, 100);
    assert_eq!(b1.intent, BroomIntent::FetchWater);

    assert_eq!(b2.name, "Hokey II");
    assert_eq!(b2.height, 30);
    assert_eq!(b2.health, 100);
    assert_eq!(b2.intent, BroomIntent::FetchWater);
}