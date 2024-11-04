pub struct Perk{
    pub icon: char,
    name: String,
    description: String,
    level: usize,
    factor_attack_speed: usize,
    factor_range: usize,
    factor_damage: usize,
    spezial_ability: Ability,
}

enum Ability{
    Invisible,
    Moab,
    Money
}
