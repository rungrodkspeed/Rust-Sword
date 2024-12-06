fn main() {
    let treasures = ["Gold", "Silver", "Crystal Gem", "Emerald", "Emerald"];
    let mut energy: i32 = 5;
    let mut found: bool = false;
    let mut game_over: bool = false;

    for treasure in treasures.iter() {
        if energy == 0 {
            game_over = true;
            println!("You are out of energy, Game Over!");
            break;
        } else if treasure == &"Ruby Gem" {
            found = true;
            println!("Found Ruby Gem, You win!");
            break;
        }

        energy -= 1;
    }

    if !found && !game_over {
        println!("not Found Ruby Gem, Go home!");
    }
}
