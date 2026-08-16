pub mod battle {
    pub fn generate_random_damage() -> i32 {
        rand::random_range(10..=20)
    }
}