extern crate rand;

use std::collections::HashMap;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
struct Boss {
    hp: i32,
    damage: i32
}

struct Buff<'a> {
    spell: &'a Spell,
    duration: i32
}

impl<'a> Buff<'a> {
    fn new(spell: &'a Spell, duration: i32) -> Buff {
        Buff{ spell: spell, duration: duration }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
enum Target {
    PLAYER, BOSS
}

#[derive(Debug)]
struct Spell {
    name: String,
    mana_cost: i32,
    spell_effects: Vec<SpellEffect>
}

impl Spell {
    fn new(name: &str, mana_cost: i32) -> Spell {
        Spell{ name: String::from(name), mana_cost: mana_cost, spell_effects: Vec::<SpellEffect>::new() }
    }

    fn add_spell_effect(self: &mut Spell, spell_effect: SpellEffect) {
        self.spell_effects.push(spell_effect);
    }
}

#[derive(Debug)]
struct SpellEffect {
    target: Target,
    health: i32,
    duration: i32,
    armor: i32,
    mana: i32
}

impl SpellEffect {
    fn new(target: Target, health: i32) -> SpellEffect {
        SpellEffect{ target: target, health: health, duration: 0, armor: 0, mana: 0 }
    }

    fn new_buff(target: Target, health: i32, duration: i32, armor: i32, mana: i32) -> SpellEffect {
        SpellEffect{ target: target, health: health, duration: duration, armor: armor, mana: mana }
    }
}

fn apply_single_buff(player: &mut Player, boss: &mut Boss, buff: &Buff) {
    for spell_effect in buff.spell.spell_effects.iter() {
        apply_spell_effect(player, boss, spell_effect);
    }
}

fn apply_spell_effect(player: &mut Player, boss: &mut Boss, spell_effect: &SpellEffect) {
    match spell_effect.target {
        Target::PLAYER => {
            player.hp += spell_effect.health;
            player.armor = spell_effect.armor;
            player.mana += spell_effect.mana;
        },
        Target::BOSS => {
            boss.hp += spell_effect.health;
        }
    }
}

fn apply_buffs<'a>(player: &mut Player, boss: &mut Boss, buffs: &mut Vec<Buff<'a>>) {
    player.armor = 0;
    for buff in buffs.iter_mut() {
        buff.duration -= 1;
        apply_single_buff(player, boss, buff);
    }

    buffs.retain(|buff| buff.duration > 0);
}

fn buff_is_active(buffs: &Vec<Buff>, name: &String) -> bool {
    buffs.iter().filter(|buff| buff.spell.name == *name).collect::<Vec<_>>().len() > 0
}

fn choose_spell<'a>(spells: &'a HashMap<String, Spell>, spell_keys: &Vec<String>, player: &Player, boss: &Boss, buffs: &Vec<Buff>) -> &'a Spell {
    if player.mana >= 229 && player.mana <= 229 + 173 && !buff_is_active(buffs, &String::from("recharge")) {
        spells.get("recharge").unwrap()
    } else if !buff_is_active(buffs, &String::from("shield")) {
        spells.get("shield").unwrap()
    } else {
        let mut rng = rand::thread_rng();
        let mut rand_index = 0;
        loop {
            rand_index = Range::new(0, spell_keys.len()).ind_sample(&mut rng);
            let spell = spells.get(&spell_keys[rand_index]).unwrap();
            if spell.spell_effects.iter().filter(|effect| effect.duration > 0).collect::<Vec<_>>().len() == 0 || !buff_is_active(buffs, &spell.name) {
                break
            }
        }

        spells.get(&spell_keys[rand_index]).unwrap()
    }
}

fn reset(player: &mut Player, boss: &mut Boss, buffs: &mut Vec<Buff>) {
    player.hp = 50;
    player.mana = 500;
    boss.hp = 71;
    buffs.clear();
}

fn main() {
    let mut boss = Boss{ hp: 71, damage: 10 };
    let mut player = Player::new();

    let mut magic_missle = Spell::new("magic_missle", 53);
    magic_missle.add_spell_effect(SpellEffect::new(Target::BOSS, -4));

    let mut drain = Spell::new("drain", 73);
    drain.add_spell_effect(SpellEffect::new(Target::BOSS, -2));
    drain.add_spell_effect(SpellEffect::new(Target::PLAYER, 2));

    let mut shield = Spell::new("shield", 113);
    shield.add_spell_effect(SpellEffect::new_buff(Target::PLAYER, 0, 6, 7, 0));

    let mut poison = Spell::new("poison", 173);
    poison.add_spell_effect(SpellEffect::new_buff(Target::BOSS, -3, 6, 0, 0));

    let mut recharge = Spell::new("recharge", 229);
    recharge.add_spell_effect(SpellEffect::new_buff(Target::PLAYER, 0, 5, 0, 101));

    let mut spells = HashMap::<String, Spell>::new();
    let spell_keys = vec![
        magic_missle.name.clone(), drain.name.clone(), shield.name.clone(), poison.name.clone(), recharge.name.clone()
    ];
    spells.insert(magic_missle.name.clone(), magic_missle);
    spells.insert(drain.name.clone(), drain);
    spells.insert(shield.name.clone(), shield);
    spells.insert(poison.name.clone(), poison);
    spells.insert(recharge.name.clone(), recharge);
    let mut buffs = Vec::<Buff>::new();
    let mut mana = 0;
    loop {
        if player.hp > 0 {
            apply_buffs(&mut player, &mut boss, &mut buffs);
            let spell = choose_spell(&spells, &spell_keys, &player, &boss, &buffs);
            for spell_effect in spell.spell_effects.iter() {
                if spell_effect.duration > 0 {
                    let buff = Buff::new(spell, spell_effect.duration);
                    buffs.push(buff);
                } else {
                    apply_spell_effect(&mut player, &mut boss, spell_effect);
                }
            }
            player.mana -= spell.mana_cost;
            if player.mana < 0 {
                continue
            }
            mana += spell.mana_cost;
            if boss.hp > 0 {
                let mut damage = boss.damage - player.armor;
                if damage < 1 {
                    damage = 1;
                }
                player.hp -= damage;
            } else {
                println!("{:?}", mana);
            }
        } else {
            reset(&mut player, &mut boss, &mut buffs);
            mana = 0;
        }
    }
}
