fn main() {
    let map = String::from("Old map");

    let borrowed_map = &map;

    let mut crabby_map = borrowed_map.to_string();

    crabby_map.push_str(" to new map");

    println!("crappy_map: {}", crabby_map);
}
