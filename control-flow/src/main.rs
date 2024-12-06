// fn main() {
//     let weather: &str = "rainy";

//     if weather == "sunday" {
//         println!("Crabby will cross the river by swimming!");
//     } else if weather == "rainy" {
//         println!("Crabby will build a bridge to stay dry.");
//     } else {
//         println!("Crabby will wait for better weather.");
//     }
// }

// fn main() {
//     let monster: &str = "dragon";

//     match monster {
//         "goblin" => println!("Crabby use his rusty sword to attack."),
//         "troll" => println!("Crabby sets a trap!"),
//         "dragon" => println!("Crabby runs for cover!"),
//         _ => println!("Crabby is confuse.."),
//     };
// }

fn main() {
    let mut wood: i32 = 0;

    loop {
        wood += 1;

        println!("Crabby gathered {} pieces of wood.", wood);

        if wood == 10 {
            println!("Crabby finished the boat!");
            break;
        }
    }
}
