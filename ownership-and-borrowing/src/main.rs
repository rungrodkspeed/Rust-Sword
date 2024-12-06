fn main() {
    let mut treasure: String = String::from("gold coins");

    let friend1: &str = &treasure;
    let friend2: &str = &treasure;

    println!("Friend 1 sees: {}", friend1);
    println!("Friend 2 sees: {}", friend2);

    let trusted_friend = &mut treasure;

    trusted_friend.push_str(" and silver coins");
    println!("Trusted friend updates: {}", trusted_friend);
}