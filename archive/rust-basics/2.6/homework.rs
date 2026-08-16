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
    if attack_power > defense {
        attack_power - defense
    } else {
        0
    }
}

fn main() {
    let person_name = "good man";
    let person_hp = 100;
    let gold = 30;
    let monster_name = "bad bird";
    let mut monster_hp = 80;

    let damage = calculate_damage(35, 12);

    show_player(person_name, person_hp, gold);
    show_monster(monster_name, monster_hp);

    for turn in 1..=3 {
        println!("training turn: {}", turn);
    }

    let mut attack_count = 0;

    let total_attacks = loop {
        monster_hp -= damage;
        attack(monster_name, person_name, damage);
        show_monster(monster_name, monster_hp);

        attack_count += 1;
        if monster_hp <= 0 {
            break attack_count;
        }
    };

    println!("total_attacks: {}", total_attacks);

    println!("bad bird defeated");

    for reward_index in 0..3 {
        if reward_index == 1 {
            continue;
        }
        println!("reward index: {}", reward_index);
    }
}
