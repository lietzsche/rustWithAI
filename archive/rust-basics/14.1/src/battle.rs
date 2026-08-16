pub fn generate_random_damage() -> i32 {
    rand::random_range(10..=20)
}

pub fn calculate_damage(attack: i32, defense: i32) -> i32 {
    if attack > defense {
        attack - defense
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_damage;

    #[test]
    fn 공격력이_방어력보다_높을_때_피해량_계산() {
        let result = calculate_damage(30, 10);
        assert_eq!(result, 20);
    }

    #[test]
    fn 공격력이_방어력보다_낮을_때_피해량_계산() {
        let result = calculate_damage(10, 30);
        assert_eq!(result, 0);
    }
}
