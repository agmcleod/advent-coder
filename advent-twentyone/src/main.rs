enum Winner {
    PLAYER, BOSS
}

#[derive(Debug)]
struct Item {
    pub cost: usize,
    pub damage: i32,
    pub armor: i32
}

impl Item {
    pub fn new(cost: usize, damage: i32, armor: i32) -> Item {
        Item{ cost: cost, damage: damage, armor: armor}
    }
}

struct Character {
    pub hp: i32,
    damage: i32,
    armor: i32
}

impl Character {
    pub fn new(hp: i32, damage: i32, armor: i32) -> Character {
        Character{ hp: hp, damage: damage, armor: armor}
    }
}

#[derive(Debug)]
struct BuyItemState {
    pub use_armor: bool,
    pub weapon_index: usize,
    pub armor_index: usize,
    pub rings_index_one: usize,
    pub rings_index_two: usize,
    pub ring_count: usize,
    pub done: bool
}

impl BuyItemState {
    pub fn new() -> BuyItemState {
        BuyItemState{ use_armor: false, weapon_index: 0, armor_index: 0, rings_index_one: 0, rings_index_two: 0, ring_count: 0, done: false }
    }
}

fn bump_weapons_or_armor<'a>(buy_item_state: &mut BuyItemState, armor: &'a Vec<Item>) {
    if buy_item_state.use_armor {
        buy_item_state.armor_index += 1;

        if buy_item_state.armor_index == armor.len() {
            buy_item_state.armor_index = 0;
            buy_item_state.weapon_index += 1;
        }
    } else {
        buy_item_state.weapon_index += 1;
    }
}

fn buy_items<'a>(weapons: &'a Vec<Item>, armor: &'a Vec<Item>, rings: &'a Vec<Item>, buy_item_state: &mut BuyItemState) -> Vec<&'a Item> {
    let mut items = Vec::<&Item>::new();

    if buy_item_state.ring_count == 1 && buy_item_state.rings_index_one == rings.len() {
        buy_item_state.rings_index_one = 0;
        bump_weapons_or_armor(buy_item_state, armor);
    }

    if buy_item_state.ring_count == 2 && buy_item_state.rings_index_one == rings.len() && buy_item_state.rings_index_two == rings.len() {
        bump_weapons_or_armor(buy_item_state, armor);
    }



    if buy_item_state.weapon_index == weapons.len() {
        if buy_item_state.ring_count < 2 {
            if !buy_item_state.use_armor {
                buy_item_state.use_armor = true;
            } else {
                buy_item_state.use_armor = false;
                buy_item_state.ring_count += 1;
            }
        } else {
            buy_item_state.done = true;
        }

        buy_item_state.armor_index = 0;
        buy_item_state.weapon_index = 0;
        buy_item_state.rings_index_one = 0;
        buy_item_state.rings_index_two = 1;
        items
    } else {
        items.push(weapons.get(buy_item_state.weapon_index).unwrap());
        if buy_item_state.use_armor {
            items.push(armor.get(buy_item_state.armor_index).unwrap());
        }
        if buy_item_state.ring_count >= 1 {
            items.push(rings.get(buy_item_state.rings_index_one).unwrap());
            buy_item_state.rings_index_one += 1;
        }
        if buy_item_state.ring_count == 2 {
            items.push(rings.get(buy_item_state.rings_index_two).unwrap());
            buy_item_state.rings_index_two += 1;
        }
        items
    }
}

fn calculate_damage(attacker_damage: &i32, defender_armor: &i32) -> i32 {
    let mut damage = attacker_damage - defender_armor;
    if damage < 1 {
        damage = 1;
    }

    damage
}

fn run_battle(player: &Character, boss: &Character, items: &Vec<&Item>) -> Winner {
    let mut player_hp = player.hp.clone();
    let player_damage = items.iter().fold(0, |sum, i| { sum + i.damage });
    let player_armor = items.iter().fold(0, |sum, i| { sum + i.armor });

    let mut boss_hp = boss.hp.clone();
    let mut result: Winner;

    loop {
        boss_hp -= calculate_damage(&player_damage, &boss.armor);
        if boss_hp > 0 {
            player_hp -= calculate_damage(&boss.damage, &player_armor);
            if player_hp <= 0 {
                result = Winner::BOSS;
                break;
            }
        } else {
            result = Winner::PLAYER;
            break;
        }
    }

    result
}

fn sort_by_lowest_cost(items: &mut Vec<Item>) {
    items.sort_by(|a, b| a.cost.cmp(&b.cost));
}

fn main() {
    let boss = Character::new(100, 8, 2);

    let mut weapons = vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0)
    ];

    let mut armor = vec![
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5)
    ];

    let mut rings = vec![
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3)
    ];

    sort_by_lowest_cost(&mut weapons);
    sort_by_lowest_cost(&mut armor);
    sort_by_lowest_cost(&mut rings);

    let player = Character::new(100, 0, 0);
    let mut costs = Vec::<usize>::new();
    let mut buy_item_state = BuyItemState::new();
    loop {
        let items = buy_items(&weapons, &armor, &rings, &mut buy_item_state);
        match run_battle(&player, &boss, &items) {
            Winner::PLAYER => {
                costs.push(items.iter().fold(0, |sum, item| {
                    sum + item.cost
                }));
            },
            Winner::BOSS => {},
        }

        if buy_item_state.done {
            break
        }
    }

    costs.sort();
    println!("{:?}", costs);
}
