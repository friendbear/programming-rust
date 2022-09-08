mod impl_function;
mod lifetime_struct;

// 名前付きフィールド構造体
#[derive(Debug, Clone)]
struct Broom{
    name: String, 
    height: u32,
    health: u32,
    position:(f32, f32, f32),
    intent: BroomIntent
}

#[derive(Copy, Clone, Debug)]
enum BroomIntent { FetchWater, DumpWater }

// Tuple型構造体
#[derive(Debug)]
struct Bounds(usize, usize);

fn main() {
    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater
    };

    let (hokey1, hokey2) = chop(hokey);
    dbg!(&hokey1);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey1.health, 100);
    

    dbg!(&hokey2);
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey2.health, 100);

    let bounds = Bounds(1024,748);
    dbg!(bounds);

    test_queue();
}

fn chop(b: Broom) -> (Broom, Broom) {

    let mut broom1 = Broom { height: b.height/2, .. b};

    // Stringはコピーではないので`broom1`は`b`の名前の所有権を得る
    let mut broom2 = Broom{ name: broom1.name.clone(), .. broom1};
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

use impl_function::Queue;

fn test_queue() {
    let mut q = Queue::<Vec<char>>{older: Vec::new(), younger: Vec::new()};

    let mut q2 = Queue::new();
    let mut r2 = Queue::new();
    q2.push("CAD");
    r2.push(0.74);
    q2.push("BTC");
    r2.push(2737.3);
}
