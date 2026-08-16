fn show_player(name: &str, hp: i32, gold: u32) {
    println!("name: {}", name);
    println!("hp: {}", hp);
    println!("gold: {}", gold);
}

fn show_monster(monster_name: &str, hp: i32) {
    println!("monster_name: {}", monster_name);
    println!("hp: {}", hp);
}

fn attack(name: &str, target_name: &str, damage: i32) {
    println!("name: {}", name);
    println!("target_name: {}", target_name);
    println!("damage: {}", damage);
}

fn calculate_damage(attack_power: i32, defense: i32) -> i32 {
    attack_power - defense
}

fn main() {
    let person_name = "good man";
    let person_hp = 100;
    let gold = 30;
    let monster_name = "bad bird";
    let monster_hp = 80;

    let damage = calculate_damage(35, 12);

    show_player(person_name, person_hp, gold);
    show_monster(monster_name, monster_hp);
    attack(monster_name, person_name, damage);
}