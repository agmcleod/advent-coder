use std::collections::HashMap;

struct Boss {
    hp: i32,
    damage: i32
}

struct Buff<'a> {
    buff_name: String,
    spell_effect: &'a SpellEffect,
    duration: i32
}

struct Player {
    damage: i32,
    armor: i32,
    hp: i32,
    mana: i32
}

impl Player {
    fn new() -> Player {
        Player{ damage: 0, armor: 0, hp: 50, mana: 500 }
    }
}

enum Target {
    PLAYER, BOSS
}

struct Spell {
    spell_effects: Vec<SpellEffect>,
    mana_cost: i32
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
    target: Target,
    damage: i32,
    duration: i32,
    armor: i32,
    mana: i32
}

impl SpellEffect {
    fn new(target: Target, damage: i32) -> SpellEffect {
        SpellEffect{ target: target, damage: damage, duration: 0, armor: 0, mana: 0 }
    }

    fn new_buff(target: Target, damage: i32, duration: i32, armor: i32, mana: i32) -> SpellEffect {
        SpellEffect{ target: target, damage: damage, duration: duration, armor: armor, mana: mana }
    }
}

fn apply_buffs<'a>(player: &mut Player, boss: &mut Boss, buffs: &mut Vec<Buff<'a>>) {
    player.armor = 0;
    for buff in buffs.iter_mut() {
        buff.duration -= 1;
        let spell_effect = &buff.spell_effect;
        match spell_effect.target {
            Target::PLAYER => {
                player.hp += spell_effect.damage;
                player.armor = spell_effect.armor;
            },
            Target::BOSS => {

            }
        }
    }
}

fn choose_spell<'a>(spells: &'a HashMap<String, Spell>, player: &Player, boss: &Boss) -> &'a Spell {
    if player.mana >= 229 && player.mana <= 229 + 73 {
        spells.get("recharge").unwrap()
    } else {
        spells.get("magic_missle").unwrap()
    }
}

fn set_player_start_values(player: &mut Player) {
    player.hp = 50;
    player.mana = 500;
}

fn take_player_turn(player: &mut Player, boss: &mut Boss, spell: &Spell) {

}

fn main() {
    let boss = Boss{ hp: 71, damage: 10 };
    let mut player = Player::new();

    let mut magic_missle = Spell::new(53);
    magic_missle.add_spell_effect(SpellEffect::new(Target::BOSS, -4));

    let mut drain = Spell::new(73);
    drain.add_spell_effect(SpellEffect::new(Target::BOSS, -2));
    drain.add_spell_effect(SpellEffect::new(Target::PLAYER, 2));

    let mut shield = Spell::new(113);
    shield.add_spell_effect(SpellEffect::new_buff(Target::PLAYER, 0, 6, 7, 0));

    let mut poison = Spell::new(173);
    poison.add_spell_effect(SpellEffect::new_buff(Target::BOSS, -3, 6, 0, 0));

    let mut recharge = Spell::new(229);
    recharge.add_spell_effect(SpellEffect::new_buff(Target::PLAYER, 0, 5, 0, 101));

    let mut spells = HashMap::<String, Spell>::new();
    spells.insert(String::from("magic_missle"), magic_missle);
    spells.insert(String::from("drain"), drain);
    spells.insert(String::from("shield"), shield);
    spells.insert(String::from("poision"), poison);
    spells.insert(String::from("recharge"), recharge);
}
