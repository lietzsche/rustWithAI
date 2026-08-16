struct BattleRecord {
    turn: u32,
    damage: i32,
    critical: bool,
}

enum Action { //Attack, Rest, Exit는 각각 Action의 variant
    Attack(i32), //tuple-like variant
    Rest { //struct-like variant
        recovered_hp: i32,
    },
    Exit, //unit-like variant
}

struct Player {
    name: String,
    hp: i32,
    gold: u32,
}

impl Player {
    fn new(name: String) -> Player { //self를 받지 않는 associated function
        Player {
            name: name,
            hp: 100,
            gold: 30,
        }
    }

    fn show_status(&self) {
        println!("name: {}", self.name);
        println!("hp: {}", self.hp);
        println!("gold: {}", self.gold);
    }

    fn take_damage(&mut self, damage: i32) { //hp가 변경되어야 해서 수정 권한을 받으려고 &mut self를 선택
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
    }

    fn is_alive(&self) -> bool { //hp가 0보다 큰지 상태만 확인하고 변경할 필요는 없어서 &self를 선택
        self.hp > 0
    }
}

struct Monster {
    name: String,
    hp: i32,
    defense: i32,
}

struct Reward {
    item: Item, //reward가 item을 field로 소유한다.
    gold: u32,
}

impl Reward {
    fn show(&self) {
        println!("item name: {}", self.item.name);
        println!("item power: {}", self.item.power);
        println!("reward gold: {}", self.gold);
    }
}

struct Item {
    name: String,
    power: i32,
}

fn main() {
    let battle_start_message: &str = "battle started";
    let victory_message: &str = "bad bird defeated";
    let reward_message: &str = "rewards received";

    let mut player = Player::new("good".to_string()); //아직 instance가 없으므로 여기서 instance 생성, instance를 만들어서 일일이 규격을 만들기 보다, 정해진 규격대로 일관된 생성 가능

    player.name.push(' ');
    player.name.push_str("man");

    let monster = Monster {
        name: "bad bird".to_string(),
        hp: 80,
        defense: 12,
    };

    let monster_attack_powers: [i32; 3] = [18, 25, 32]; //Stack

    show_message(battle_start_message);

    println!("monster count: {}", monster_attack_powers.len());
    println!("first monster attack: {}", monster_attack_powers[0]);
    println!("third monster attack: {}", monster_attack_powers[2]);

    let mut monster_hp = [monster.hp; 3]; //Stack

    println!("second monster hp: {}", monster_hp[1]);

    let damage = calculate_damage(35, monster.defense); //Stack

    player.show_status();
    player.take_damage(35);
    player.show_status();
    println!("player is alive: {}", player.is_alive());
    show_monster(&monster.name, monster_hp[0]);

    for turn in 1..=3 {
        println!("training turn: {}", turn);
    }

    let mut attack_count = 0; //Stack

    let total_attacks = loop { //Stack
        monster_hp[0] -= damage;
        attack(&monster.name, &player.name, damage);
        show_monster(&monster.name, monster_hp[0]);

        attack_count += 1; //Stack
        if monster_hp[0] <= 0 {
            break attack_count;
        }
    };

    println!("total_attacks: {}", total_attacks);

    println!("player name still available: {}", player.name);

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

    let mut upgrade_target: String = String::from("fire staff");
    upgrade_weapon(&mut upgrade_target);
    println!("upgrade_target: {}", upgrade_target); //소유권은 그대로 가지고 있다.

    let mut special_weapon: String = String::from("storm blade");
    let first_borrow = &mut special_weapon;
    first_borrow.push_str(" +1");
    let second_borrow = &mut special_weapon; //첫 borrow의 마지막 사용 이후에 두 번째 borrow가 시작된다, 이후로 다시 첫 borrow가 사용되면 컴파일 오류 처리
    second_borrow.push_str(" +2");
    println!("special_weapon: {}", special_weapon);

    let damages = [10, 20, 30, 40, 50];
    let damage_slice: &[i32] = &damages[1..4];
    show_damage_slice(damage_slice);

    let title = String::from("Rust Knight");
    let title_slice: &str = &title[0..4];
    show_title_slice(title_slice);
    println!("original title: {}", title);

    let mut quest_inventory: Vec<String> = vec![];
    add_item(&mut quest_inventory, "potion".to_string());
    add_item(&mut quest_inventory, "steel shield".to_string());
    let visible_items: &[String] = &quest_inventory[..];
    show_inventory(visible_items);
    add_item(&mut quest_inventory, "dungeon key".to_string());
    show_inventory(&quest_inventory);

    let item = Item {
        name: "ancient sword".to_string(),
        power: 45,
    };

    let reward = Reward {
        item: item,
        gold: 50,
    };

    reward.show(); //&self만 사용하므로 호출 후에도 reward.gold를 사용할 수 있다
    
    player.gold += reward.gold;
    player.show_status();

    let available_actions: [Action; 3] = [Action::Attack(25), Action::Rest {recovered_hp: 15}, Action::Exit];
    println!("available_actions count: {}", available_actions.len());
    let first_action_index: usize = 0;
    println!("first_action_index: {}", first_action_index);

    for available_action in &available_actions {
        handle_action(available_action);
    }
    for available_action in &available_actions {
        inspect_attack(available_action);
    }
    println!("available_actions count: {}", available_actions.len());

    let battle_record = BattleRecord {
        turn: 1,
        damage: 25,
        critical: true,
    };
    show_battle_record(&battle_record);
    println!("battleRecord turn: {}", battle_record.turn);

    let equipped_weapon: Option<String> = Some(String::from("steel sword"));
    show_equipped_weapon(&equipped_weapon);
    let empty_weapon: Option<String> = None;
    show_equipped_weapon(&empty_weapon);
    println!("original option: {:?}", equipped_weapon);

    let success_result = load_game(true);
    let failure_result = load_game(false);
    show_load_result(&success_result);
    show_load_result(&failure_result);

    let successful_load = load_game(true);
    println!("unwrapped data: {}", successful_load.unwrap());

    let required_save = load_game(true);
    let expected_data = required_save.expect("required save file should exist");
    println!("expected data: {}", expected_data);

    let prepare_success = prepare_game(true);
    show_load_result(&prepare_success);
    let prepare_failure = prepare_game(false);
    show_load_result(&prepare_failure);

    let critical_bonus: i32 = 10;
    let calculate_critical_damage = |base_damage: i32| base_damage + critical_bonus;
    let critical_damage = calculate_critical_damage(23);
    println!("critical damage: {}", critical_damage);
    println!("critical bonus still available: {}", critical_bonus);
}

fn prepare_game(success: bool) -> Result<String, String> {
    let mut ok_msg = load_game(success)?;
    ok_msg.push_str(" | ready");
    Ok(ok_msg)
}

fn show_load_result(result: &Result<String, String>) {
    match result {
        Ok(ok_msg) => {
            println!("loaded data: {}", ok_msg);
        },
        Err(err_msg) => {
            println!("load error: {}", err_msg);
        },
    }
}

fn load_game(success: bool) -> Result<String, String> {
    if success {
        Ok(String::from("player level 10"))
    } else {
        Err(String::from("save file not found"))
    }
}

fn show_equipped_weapon(weapon: &Option<String>) {
    match weapon {
        Some(name) => {
            println!("equipped weapon: {}", name);
        },
        None => {
            println!("equipped weapon: none")
        },
    }
}

fn show_battle_record(battle_record: &BattleRecord) {
    let BattleRecord {
        turn,
        damage,
        critical,
    } = battle_record;
    println!("turn: {}", turn);
    println!("damage: {}", damage);
    println!("critical: {}", critical);
}

fn inspect_attack(action: &Action) {
    if let Action::Attack(amount) = action {
        println!("amount: {}", amount);
    }
}

fn handle_action(action: &Action) { //&Action을 받으므로 배열 원소의 소유권은 이동하지 않는다.
    match action { //match가 모든 Action variant를 처리한다.
        Action::Attack(amount) => { //pattern을 통해 variant 내부 데이터를 꺼낸다.
            println!("Attack amount: {}", amount);
        },
        Action::Rest {recovered_hp} => {
            println!("Rest recovered_hp: {}", recovered_hp);
        },
        Action::Exit => {
            println!("exit game");
        }
    }
}

fn show_inventory(items: &[String]) {
    println!("items length: {}", items.len());
    for item in items {
        println!("item: {}", item);
    }
}

fn add_item(inventory: &mut Vec<String>, item: String) {
    inventory.push(item);
}

fn show_damage_slice(damages: &[i32]) {
    println!("damage slice length: {}", damages.len());
    for damage in damages {
        println!("damage: {}", damage);
    }
}

fn show_title_slice(title: &str) {
    println!("title slice: {}", title);
}

fn upgrade_weapon(weapon: &mut String) { //&mut로 소유권이 아닌 변경 권한만 빌린다.
    weapon.push_str(" +1");
}

fn inspect_weapon(weapon: &String) { //immutable borrow이므로 읽기만 가능
    // weapon.push_str(" +1");
    println!("weapon: {}", weapon);
    println!("weapon length: {}", weapon.len());
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
