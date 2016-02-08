enum Winner {
    PLAYER, BOSS
}

struct Item {
    pub cost: usize,
    pub damage: usize,
    pub armor: usize
}

impl Item {
    pub fn new(cost: usize, damage: usize, armor: usize) -> Item {
        Item{ cost: cost, damage: damage, armor: armor}
    }
}

struct Character {
    pub hp: usize,
    damage: usize,
    armor: usize
}

impl Character {
    pub fn new(hp: usize, damage: usize, armor: usize) -> Character {
        Character{ hp: hp, damage: damage, armor: armor}
    }

    pub fn add_armor(self: &mut Character, armor: usize) {
        self.armor += armor;
    }

    pub fn add_damage(self: &mut Character, damage: usize) {
        self.damage += damage;
    }

    pub fn reset_stats(self: &mut Character) {
        self.damage = 0;
        self.armor = 0;
    }
}

fn buy_items(weapons: &Vec<Item>, armor: &Vec<Item>, rings: &Vec<Item>) -> Vec<Item> {
    let items = Vec::<Item>::new();

    items
}

fn run_battle(player: &Character, boss: &Character, items: &Vec<Item>) -> Winner {
    Winner::PLAYER
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

    let mut player = Character::new(100, 0, 0);
    let mut cost = 0;
    loop {
        let items = buy_items(&weapons, &armor, &rings);
        match run_battle(&player, &boss, &items) {
            Winner::PLAYER => {
                cost = items.iter().fold(0, |sum, item| {
                    sum + item.cost
                });
                break
            },
            Winner::BOSS => continue,
        }
    }

    print!("{:?}", cost);
}
