fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("Found Some Items.".to_string())
    }
}

fn open_door(is_danger: bool) -> Result<String, String> {
    if is_danger {
        Err("It's Danger!".to_string())
    } else {
        Ok("The door is safe.".to_string())
    }
}

fn main() {
    let chest_result = match open_chest(false) {
        Some(found) => found,
        None => "The chest is empty".to_string(),
    };

    let door_result = match open_door(false) {
        Ok(safe) => safe,
        Err(danger) => panic!("{}", danger),
    };

    println!("Chest result: {}", chest_result);
    println!("Door result: {}", door_result);
}
