use tokio;

async fn gather_herbs() {
    println!("Crabby is gathering herbs...");
}

async fn collect_gold_coins() {
    println!("Crabby is collecting gold coins...");
}

async fn fight_monster() {
    println!("Crabby is fighting the monster!");
}

#[tokio::main]
async fn main() {
    let task_1 = tokio::spawn(gather_herbs());
    let task_2 = tokio::spawn(collect_gold_coins());
    let task_3 = tokio::spawn(fight_monster());

    let _ = tokio::join!(task_1, task_2, task_3);
}
