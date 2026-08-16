fn main() {
    println!("rust rpg started");

    let random_damage: i32 = rand::random_range(10..=20);

    println!("random_damage: {}", random_damage);
}
