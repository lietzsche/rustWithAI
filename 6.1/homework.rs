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

fn show_item(item: &str) {
    println!("item: {}", item);
}

fn main() {
    let battle_start_message: &str = "battle started";
    let victory_message: &str = "bad bird defeated";
    let reward_message: &str = "rewards received";

    let player: (i32, u32) = (100, 30); //Stack
    
    let mut player_name = String::from("good"); //관리정보는 Stack, 내용은 Heap
    player_name.push(' ');
    player_name.push_str("man");

    let monster: (&str, i32, i32) = ("bad bird", 80, 12); //관리정보는 Stack, 문자열 리터럴은 정적 메모리
    let (monster_name, initial_hp, monster_defense) = monster; //Stack

    let monster_attack_powers: [i32; 3] = [18, 25, 32]; //Stack

    show_message(battle_start_message);

    println!("monster count: {}", monster_attack_powers.len());
    println!("first monster attack: {}", monster_attack_powers[0]);
    println!("third monster attack: {}", monster_attack_powers[2]);

    let mut monster_hp = [initial_hp; 3]; //Stack

    println!("second monster hp: {}", monster_hp[1]);

    let damage = calculate_damage(35, monster_defense); //Stack

    show_player(&player_name, player.0, player.1);
    show_monster(monster_name, monster_hp[0]);

    for turn in 1..=3 {
        println!("training turn: {}", turn);
    }

    let mut attack_count = 0; //Stack

    let total_attacks = loop { //Stack
        monster_hp[0] -= damage;
        attack(monster_name, &player_name, damage);
        show_monster(monster_name, monster_hp[0]);

        attack_count += 1; //Stack
        if monster_hp[0] <= 0 {
            break attack_count;
        }
    };

    println!("total_attacks: {}", total_attacks);

    println!("player name still available: {}", player_name);

    show_message(victory_message);

    let mut battle_log = "battle result".to_string(); //관리정보는 Stack, 내용은 Heap
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

    let mut inventory: Vec<&str> = Vec::new(); //Vec의 관리정보는 Stack, &str 원소들을 저장하는 버퍼는 Heap, 문자열 리터럴은 정적 메모리
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

    let owned_item: String = String::from("steel shield"); //관리정보는 Stack, 내용은 Heap
    let borrowed_item: &str = &owned_item; //&str는 Stack, 가리키는 owned_item의 내용 Heap

    show_item(borrowed_item);
    show_item("healing herb");

    println!("owned item still available: {}", owned_item);

    let player_title: &str = "용사A"; //&str의 관리정보는 Stack, 실제 문자열 리터럴은 정적 메모리
    println!("title byte length: {}", player_title.len());
    println!("title char count: {}", player_title.chars().count());
    for c in player_title.chars() {
        println!("title char: {}", c);
    }

    let code: &str = "RPG"; //&str의 관리정보는 Stack, 실제 문자열 리터럴은 정적 메모리
    for b in code.bytes() {
        println!("code byte: {}", b);
    }

    {
        let temporary_item: String = String::from("dungeon key");//소유자: temporary_item
        println!("temporary_item: {}", temporary_item);
    }//temporary_item의 scope는 여기까지: Heap 데이터는 여기서 정리

    let first_item: String = String::from("ancient sword");
    let equipped_item = first_item; //소유권 이동: first_item -> equipped_item
    println!("equipped_item: {}", equipped_item);
    // println!("first_item: {}", first_item);

    let original_gold: u32 = 100;
    let copied_gold = original_gold; //u32가 Copy를 구현하므로 값이 자동으로 복사
    println!("copied_gold: {}", copied_gold);
    println!("original_gold: {}", original_gold); //소유권 이전이 아니라 값이 복사되는 타입

    let original_weapon: String = String::from("silver sword");
    let cloned_weapon: String = original_weapon.clone(); //clone()이 Heap 문자열 데이터까지 복제
    println!("cloned_weapon: {}", cloned_weapon);
    println!("original_weapon: {}", original_weapon); //String::clone()으로 Heap 데이터도 복제

    let quest_weapon: String = String::from("dragon spear");
    equip_weapon(quest_weapon); //소유권 이동
    // println!("quest_weapon: {}", quest_weapon);

    let current_gold: u32 = 100;
    receive_gold(current_gold); //u32는 Copy trait를 구현하므로 복제
    println!("current_gold: {}", current_gold);

    let quest_reward = create_quest_reward(); //소유권 가져오기
    println!("quest_reward: {}", quest_reward);

    let inspection_weapon: String = String::from("ice bow"); //소유자
    inspect_weapon(&inspection_weapon); //소유권은 전달하지 않고 &로 레퍼런스만 전달
    println!("inspection_weapon: {}", inspection_weapon); //소유권이 그대로 있으므로 사용 가능
}

fn inspect_weapon(weapon: &String) {
    println!("weapon: {}", weapon);
}

fn create_quest_reward() -> String {
    String::from("legendary shield") // expression으로 반환하면서 소유권 전달
}

fn receive_gold(gold: u32) {
    println!("gold: {}", gold);
}

fn equip_weapon(weapon: String) {
    println!("weapon: {}", weapon);
}
