struct Boss {
    pub hp: i32,
    pub damage: i32
}

struct Player {
    hp: i32,
    mana: i32
}

enum Target {
    PLAYER, BOSS
}

struct Spell {
    pub spell_effects: Vec<SpellEffect>,
    pub mana_cost: i32
}

impl Spell {
    fn new(mana_cost: i32) -> Spell {
        Spell{ mana_cost: mana_cost, spell_effects: Vec::<SpellEffect>::new() }
    }

    fn add_spell_effect(self: &mut Spell, spell_effect: SpellEffect) {
        self.spell_effects.push(spell_effect);
    }
}

struct SpellEffect {
    pub target: Target,
    pub damage: i32,
    pub duration: i32,
    pub armor: i32,
    pub mana: i32
}

impl SpellEffect {
    fn new(target: Target, damage: i32) -> SpellEffect {
        SpellEffect{ target: target, damage: damage, duration: 0, armor: 0, mana: 0 }
    }

    fn new_buff(target: Target, damage: i32, duration: i32, armor: i32, mana: i32) -> SpellEffect {
        SpellEffect{ target: target, damage: damage, duration: duration, armor: armor, mana: mana }
    }
}

fn set_player_start_values(player: &mut Player) {
    player.hp = 50;
    player.mana = 500;
}

fn main() {
    let boss = Boss{ hp: 71, damage: 10 };
    let mut player = Player{hp: 0, mana: 0};

    set_player_start_values(&mut player);

    let mut magic_missle = Spell::new(53);
    magic_missle.add_spell_effect(SpellEffect::new(Target::BOSS, -4));

    let mut drain = Spell::new(73);
    drain.add_spell_effect(SpellEffect::new(Target::BOSS, -2));
    drain.add_spell_effect(SpellEffect::new(Target::PLAYER, 2));

    let mut shield = Spell::new(113);
    shield.add_spell_effect(SpellEffect::new_buff(Target::PLAYER, 0, 6, 7, 0));

    let mut poison = Spell::new(173);
    poison.add_spell_effect(SpellEffect::new_buff(Target::BOSS, 3, 6, 0, 0));

    let mut recharge = Spell::new(229);
    recharge.add_spell_effect(SpellEffect::new_buff(Target::PLAYER, 0, 5, 0, 101));

    let mut spells = vec![magic_missle, drain, shield, poison, recharge];
}
