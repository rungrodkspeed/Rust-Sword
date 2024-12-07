fn main() {
    let mut treasures = vec!["Gold coins", "Diamond Gems", "Sword"];

    println!("Treasures :{:?}", treasures);
    println!("Length of treasures: {}", treasures.len());
    println!("Capacity of treasures: {}", treasures.capacity());

    treasures.push("Ruby Gems");

    println!("Treasures :{:?}", treasures);
    println!("Length of treasures: {}", treasures.len());
    println!("Capacity of treasures: {}", treasures.capacity());

    treasures.remove(treasures.len() - 1);
    println!("Treasures :{:?}", treasures);
    println!("Length of treasures: {}", treasures.len());
    println!("Capacity of treasures: {}", treasures.capacity());
}
