use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Fruit {
    Apple,
    Banana,
    Orange,
}

struct FruitStand {
    fruit: HashMap<Fruit, u32>,
}

impl IntoIterator for FruitStand {
    type Item = (Fruit, u32);
    type IntoIter = std::collections::hash_map::IntoIter<Fruit, u32>;

    // items wil be moved into this `self`
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.into_iter() // convention is into_iter is used for moving
    }
}

impl<'a> IntoIterator for &'a FruitStand {
    type Item = (&'a Fruit, &'a u32);
    type IntoIter = std::collections::hash_map::Iter<'a, Fruit, u32>;

    // items wil be borrowed into this because of lifetime annotation
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter() // convention is iter is for borrowing
    }
}

impl<'a> IntoIterator for &'a mut FruitStand {
    type Item = (&'a Fruit, &'a mut u32);
    type IntoIter = std::collections::hash_map::IterMut<'a, Fruit, u32>;

    // items wil be borrowed into this because of lifetime annotation and will be mutable
    fn into_iter(self) -> Self::IntoIter {
        self.fruit.iter_mut() // convention is iter is for borrowing and being mutable
    }
}

pub fn start() {
    let mut fruit = HashMap::new();
    fruit.insert(Fruit::Banana, 5);
    fruit.insert(Fruit::Apple, 2);
    fruit.insert(Fruit::Orange, 6);

    let fruit = fruit;

    let mut store = FruitStand { fruit };

    // borrowed
    for (fruit, stock) in &store {
        println!("{:?}: {:?}", fruit, stock);
    }

    // borrowed and mutable
    for (fruit, stock) in &mut store {
        println!("{:?}: {:?}", fruit, stock);
        *stock += 10;
    }

    // moved
    for (fruit, stock) in store {
        println!("{:?}: {:?}", fruit, stock);
    }
}
