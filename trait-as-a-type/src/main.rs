trait Gear {
    fn use_gear(&self);
}

struct Sword;
struct Bow;
struct Postion;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swing sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Fire arrow");
    }
}

impl Gear for Postion {
    fn use_gear(&self) {
        println!("Drink potion");
    }
}

fn use_gear<T: Gear>(item: T) {
    item.use_gear();
}

fn main() {
    let carbby_sword = Sword;
    let carbby_bow = Bow;
    let carbby_postion = Postion;

    use_gear(carbby_sword);
    use_gear(carbby_bow);
    use_gear(carbby_postion);
}
