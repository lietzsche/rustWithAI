fn main() {
    println!("rust rpg started");

    let random_damage: i32 = rand::random_range(10..=20);

    println!("random_damage: {}", random_damage);

    let package_name = env!("CARGO_PKG_NAME");
    let crate_path = module_path!();
    println!("package_name: {}", package_name);
    println!("crate_path: {}", crate_path);
}
