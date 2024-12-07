fn main() {
    let treasures = vec![100, 200, 300, 400];

    //got doubles by iterator
    let mut double_by_iter: Vec<i32> = vec![];
    for treasure in treasures.iter() {
        double_by_iter.push(treasure * 2);
    }

    println!("Double by iteration: {:?}", double_by_iter);

    //got doubles by closures
    let double_by_closure: Vec<i32> = treasures.iter().map(|x| x * 2).collect();

    println!("Double by closures: {:?}", double_by_closure);
}
