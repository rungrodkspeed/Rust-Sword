use modules_and_crates::potions::use_item;

mod weapons {
    pub fn use_item() {
        println!("You used a weapon!");
    }
}

fn main() {
    use_item::use_item();
    weapons::use_item();
}
