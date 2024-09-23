pub fn calculate_damage(base_attack: i32, defense: i32) -> i32 {
    (base_attack - defense).max(1)
}
