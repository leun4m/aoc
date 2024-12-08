use itertools::Itertools;
use log::debug;

pub fn solve(input: &str) {
    let boss = parse(input);
    println!("Part 1: {}", part_one(&boss));
    println!("Part 2: {}", part_two(&boss));
}

const PLAYER_HP: i32 = 100;

const STR_HIT_POINTS: &str = "Hit Points:";
const STR_DAMAGE: &str = "Damage:";
const STR_ARMOR: &str = "Armor:";

fn parse(input: &str) -> Player {
    let mut hit_points = 0;
    let mut damage = 0;
    let mut armor = 0;

    for line in input.lines() {
        if line.starts_with(STR_HIT_POINTS) {
            hit_points = line.replace(STR_HIT_POINTS, "").trim().parse().unwrap();
        }

        if line.starts_with(STR_DAMAGE) {
            damage = line.replace(STR_DAMAGE, "").trim().parse().unwrap();
        }

        if line.starts_with(STR_ARMOR) {
            armor = line.replace(STR_ARMOR, "").trim().parse().unwrap();
        }
    }

    Player {
        hit_points,
        damage,
        armor,
    }
}

fn part_one(boss: &Player) -> i32 {
    let mut gold = 0;
    let mut has_won = false;
    let all_combos = all_combinations();
    debug!("all_combos: {}", all_combos.len());
    while !has_won {
        gold += 1;
        let combis = combinations(gold, &all_combos);

        debug!("Gold: {gold}\tCombis: {}", combis.len());
        let mut min_hp = 200;
        for combi in combis {
            let mut player = Player {
                hit_points: PLAYER_HP,
                damage: damage(&combi),
                armor: armor(&combi),
            };

            if fight(&mut player, &mut boss.clone()) {
                has_won = true;
            }
            min_hp = std::cmp::min(boss.hit_points, min_hp);
        }
    }

    gold
}

fn part_two(boss: &Player) -> i32 {
    all_combinations()
        .iter()
        .filter(|(combi, _)| {
            let mut player = Player {
                hit_points: PLAYER_HP,
                damage: damage(&combi),
                armor: armor(&combi),
            };

            !fight(&mut player, &mut boss.clone())
        })
        .map(|(_, cost)| *cost)
        .max()
        .unwrap()
}

fn combinations(gold: i32, all_combos: &[(Vec<Item>, i32)]) -> Vec<&Vec<Item>> {
    let mut combos = Vec::new();
    for (combo, cost) in all_combos {
        if *cost > gold {
            break;
        }

        combos.push(combo);
    }
    combos
}

fn all_combinations() -> Vec<(Vec<Item>, i32)> {
    ring_combinations()
        .iter()
        .cartesian_product(WEAPONS.iter())
        .cartesian_product(ARMOR.iter())
        .map(|((rings, weapon), armor)| {
            let mut items = rings.to_owned();
            items.push(*weapon);
            items.push(*armor);
            items
        })
        .map(|items| {
            let gold = items.iter().map(|item| item.cost).sum::<i32>();
            (items, gold)
        })
        .sorted_by_key(|(_, gold)| *gold)
        .collect()
}

fn ring_combinations() -> Vec<Vec<Item>> {
    RINGS
        .iter()
        .cartesian_product(RINGS.iter())
        .filter(|(r1, r2)| r1.name != r2.name)
        .map(|(r1, r2)| vec![*r1, *r2])
        .collect()
}

fn fight(player: &mut Player, boss: &mut Player) -> bool {
    let mut players_turn = true;
    while player.hit_points > 0 && boss.hit_points > 0 {
        if players_turn {
            boss.hit_points -= std::cmp::max(1, player.damage - boss.armor);
        } else {
            player.hit_points -= std::cmp::max(1, boss.damage - player.armor);
        }

        players_turn = !players_turn;
    }

    boss.hit_points < 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Player {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

fn damage(items: &[Item]) -> i32 {
    items.iter().map(|x| x.damage).sum()
}

fn armor(items: &[Item]) -> i32 {
    items.iter().map(|x| x.armor).sum()
}

#[derive(Debug, Clone, Copy)]
struct Item {
    name: &'static str,
    cost: i32,
    damage: i32,
    armor: i32,
}

const WEAPONS: [Item; 5] = [
    Item {
        name: "Dagger",
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        name: "Shortsword",
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        name: "Warhammer",
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        name: "Longsword",
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        name: "Greataxe",
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMOR: [Item; 6] = [
    Item {
        name: "NOTHING",
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        name: "Leather",
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Chainmail",
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Splintmail",
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        name: "Bandedmail",
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        name: "Platemail",
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 8] = [
    Item {
        name: "NOTHING",
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        name: "NOTHING 2",
        cost: 0,
        damage: 0,
        armor: 0,
    },
    Item {
        name: "Damage +1",
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        name: "Damage +2",
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        name: "Damage +3",
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        name: "Defense +1",
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        name: "Defense +2",
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        name: "Defense +3",
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Hit Points: 109
Damage: 8
Armor: 2";

    #[test]
    fn test_parse() {
        assert_eq!(
            Player {
                hit_points: 109,
                damage: 8,
                armor: 2
            },
            parse(EXAMPLE_INPUT)
        );
    }

    #[test]
    fn test_fight() {
        let mut player = Player {
            hit_points: 8,
            damage: 5,
            armor: 5,
        };
        let mut boss = Player {
            hit_points: 12,
            damage: 7,
            armor: 2,
        };
        assert_eq!(true, fight(&mut player, &mut boss));
    }
}
