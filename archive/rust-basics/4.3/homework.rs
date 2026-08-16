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

fn show_message(message: &str) {
    println!("message: {}", message);
}

fn main() {
    let battle_start_message: &str = "battle started";
    let victory_message: &str = "bad bird defeated";
    let reward_message: &str = "rewards received";

    let player: (i32, u32) = (100, 30);
    
    let mut player_name = String::from("good");
    player_name.push(' ');
    player_name.push_str("man");

    let monster: (&str, i32, i32) = ("bad bird", 80, 12);
    let (monster_name, initial_hp, monster_defense) = monster;

    let monster_attack_powers: [i32; 3] = [18, 25, 32];

    show_message(battle_start_message);

    println!("monster count: {}", monster_attack_powers.len());
    println!("first monster attack: {}", monster_attack_powers[0]);
    println!("third monster attack: {}", monster_attack_powers[2]);

    let mut monster_hp = [initial_hp; 3];

    println!("second monster hp: {}", monster_hp[1]);

    let damage = calculate_damage(35, monster_defense);

    show_player(&player_name, player.0, player.1);
    show_monster(monster_name, monster_hp[0]);

    for turn in 1..=3 {
        println!("training turn: {}", turn);
    }

    let mut attack_count = 0;

    let total_attacks = loop {
        monster_hp[0] -= damage;
        attack(monster_name, &player_name, damage);
        show_monster(monster_name, monster_hp[0]);

        attack_count += 1;
        if monster_hp[0] <= 0 {
            break attack_count;
        }
    };

    println!("total_attacks: {}", total_attacks);

    println!("player name still available: {}", player_name);

    show_message(victory_message);

    let mut battle_log = "battle result".to_string();
    battle_log.push(':');
    battle_log.push(' ');
    battle_log.push_str("victory");
    println!("battle log: {}", battle_log);

    for reward_index in 0..3 {
        if reward_index == 1 {
            continue;
        }
        println!("reward index: {}", reward_index);
    }

    let mut inventory: Vec<&str> = Vec::new();
    inventory.push("potion");
    inventory.push("rusty sword");
    inventory.push("gold coin");

    println!("inventory count: {}", inventory.len());
    println!("first item: {}", inventory[0]);

    inventory.pop();

    println!("inventory count after pop: {}", inventory.len());

    let selected_index: usize = 1;
    println!("selected item: {}", inventory[selected_index]);

    let inventory_count: usize = inventory.len();
    println!("inventory count as usize: {}", inventory_count);

    for index in 0..inventory.len() {
        println!("inventory[{}]: {}", index, inventory[index]);
    }

    for attack_power in monster_attack_powers {
        println!("monster attack: {}", attack_power);
    }

    for item in &inventory {
        println!("inventory item: {}", item);
    }

    println!("inventory still has: {}", inventory.len());

    show_message(reward_message);
}
