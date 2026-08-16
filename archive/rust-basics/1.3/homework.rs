fn main() {
    let name = "김이름";
    let mut hp = 100;
    let gold: u32 = 15;
    let power: i64 = 10000;
    let critical_chance = 0.4;
    let is_alive = true;
    let grade = 'A';
    let inventory_number: usize = 3;

    println!("name: {}", name);
    println!("hp: {}", hp);
    println!("gold: {}", gold);
    println!("power: {}", power);
    println!("critical_chance: {}", critical_chance);
    println!("is_alive: {}", is_alive);
    println!("grade: {}", grade);
    println!("inventory_number: {}", inventory_number);

    hp = 85;
    println!("hp: {}", hp);
}