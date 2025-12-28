use std::fmt::Display;

use super::player::Player;
use super::utilities::*;

#[derive(PartialEq, Clone)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub item_type: ItemType,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ItemType {
    Weapon { damage: u16 },
    Healer { amount: u16 },
    Armor { reduction: f64 },
    Key,
    Special { function: fn(&mut Player) }, // experimental
}


impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Weapon { .. } => write!(f, "Weapon"),
            ItemType::Healer { .. } => write!(f, "Healer"),
            ItemType::Armor { .. } => write!(f, "Armor"),
            ItemType::Key => write!(f, "Key"),
            ItemType::Special { .. } => write!(f, "Special"),
        }
    }
}

pub type Drops = Vec<(Item, f64, bool)>; // Item, Chance (e.g., 0.01 = 1%), Duplicates allowed

impl Item {
    pub fn new(name: &str, desc: &'static str, item_type: ItemType) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            item_type,
        }
    }

    pub fn use_item(&self, plr: &mut Player, loc: usize) {
        match self.item_type {
            ItemType::Healer { amount } => drop(plr.heal(amount)),
            ItemType::Special { function } => function(plr),
            except => return eprintln!("Attempted to use a `{}` item", except),
        };
        plr.remove_from_inventory(loc, 1);
    }

    /// Odds add up to .60 (60%) currently
    pub fn get_base_drops() -> Drops {
        vec![
            // total c = 60
            (
                Item::new("Apple", "Crunchy :3", ItemType::Healer { amount: 25 }),
                0.39,
                true,
            ),
            (
                Item::new(
                    "Fairy Milk Bottle",
                    "The source? You're asking too many questions...",
                    ItemType::Healer { amount: 40 },
                ),
                0.10,
                true,
            ),
            (
                Item::new(
                    "Magic Tea",
                    "Increases max HP +15 and heals by the same amount",
                    ItemType::Special {
                        function: |plr| {
                            plr.max_health += 15;
                            plr.heal(15);
                        },
                    },
                ),
                0.05,
                true,
            ),
            (
                Item::new(
                    "XP Potion",
                    "Increases XP gain +10%",
                    ItemType::Special {
                        function: |plr| plr.xp_multiplier += 0.1,
                    },
                ),
                0.05,
                true,
            ),
            (
                Item::new("Tranquility Stone", "Meditate instantly", ItemType::Key),
                0.01,
                false,
            ),
        ]
    }

    pub fn roll_drop(drops: Drops, plr: &Player) -> Option<Item> {
        let proc: Vec<_> = drops
            .into_iter()
            .filter(|(itm, _, can_dupe)| *can_dupe || !plr.has_item(&itm.name))
            .map(|(i, c, _)| (i, c))
            .collect();

        math::weigh_vec(proc)
    }
}
