enum Species {
    Finch,
    Hawk,
    Parrot,
}

struct Bird {
    age: usize,
    species: Species,
}

fn example1_bird() {
    let hawk = Bird {
        age: 13,
        species: Species::Hawk,
    };

    match hawk {
        Bird { age: 4, .. } => println!("4 years old bird"),
        Bird {
            age: 4..=10 | 15..=20,
            ..
        } => println!("4-10 or 15-20 years old"),
        Bird {
            species: Species::Finch,
            ..
        } => println!("Finch"),
        Bird { .. } => println!("other bird"),
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

fn example2_difficulty() {
    let stage = 5;
    let diff = Difficulty::Normal;
    match stage {
        s if (s == 5 && diff == Difficulty::Easy) => println!("easy mode stage 5"),
        s if diff == Difficulty::Normal => println!("normal difficulty stage {}", s),
        s @ 10 | s @ 15 => println!("stage 10 or 15 --> {}", s),
        _ => println!("stage {}", stage),
    }
}

struct Vehicle {
    km: usize,
    year: usize,
}

fn example3_vehicle() {
    let car = Vehicle {
        km: 80_000,
        year: 2020,
    };

    match car {
        Vehicle { km, year } if km == 0 && year == 2020 => println!("new 2020 model!"),
        Vehicle { km, .. } if km <= 50_000 => println!("under 50k km"),
        Vehicle { km, .. } if km >= 80_000 => println!("at least 80k km"),
        Vehicle { year, .. } if year == 2020 => println!("older models"),
        Vehicle { .. } => println!("other mileage"),
    }
}

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: Tile) {
    use Tile::*;

    match tile {
        Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {brick:?}");
        }
        Brick(other) => println!("{other:?} brick"),
        Dirt | Grass | Sand => println!("Ground tile"),

        // specifically looking for gold
        Treasure(TreasureChest {
            amount,
            content: TreasureItem::Gold,
        }) if amount >= 100 => println!("lots of gold!"),

        Water(pressure) if pressure.0 < 10 => println!("water pressure low level {}", pressure.0),
        Water(pressure) if pressure.0 >= 10 => println!("water pressure high level {}", pressure.0),

        _ => (),
    }
}

fn example4_tile() {
    let tile = Tile::Brick(BrickStyle::Red);
    print_tile(tile);

    let tile = Tile::Sand;
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 200,
    });
    print_tile(tile);

    let tile = Tile::Water(Pressure(9));
    print_tile(tile);
}

pub fn start() {
    example1_bird();
    example2_difficulty();
    example3_vehicle();
    example4_tile();
}
