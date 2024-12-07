use std::{cell::RefCell, rc::Rc};

fn main() {
    // let chest = Box::new(10);

    // println!("Gold: {}", *chest);

    // let shared_chest = Rc::new(RefCell::new(chest));

    // println!("Gold: {}", shared_chest.borrow());

    // **shared_chest.borrow_mut() += 10;
    // **shared_chest.borrow_mut() += 5;

    // println!("Gold: {}", shared_chest.borrow());

    let chest = 10;

    println!("Gold: {}", chest);

    let shared_chest = Rc::new(RefCell::new(chest));

    println!("Gold: {}", shared_chest.borrow());

    *shared_chest.borrow_mut() += 10;
    *shared_chest.borrow_mut() += 5;

    println!("Gold: {}", shared_chest.borrow());
}
