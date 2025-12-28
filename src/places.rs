use std::fmt::Display;

use super::entities::Entity;
use super::items::ItemType as IType;
use super::items::*;

pub struct Place {
    pub name: String,
    location: Location,
    pub desc: String,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Location {
    Forest,
    Mountains,
    Cave,
}

impl Location {
    pub fn req_key(&self) -> bool {
        !matches!(self, Self::Forest /* | Self::... */)
    }
}

// Really, really, useful
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forest => write!(f, "Forest"),
            Self::Cave => write!(f, "Cave"),
            Self::Mountains => write!(f, "Mountains"),
        }
    }
}

fn new_ent(n: &'static str, h: u16, d: (u16, u16)) -> Entity {
    Entity::new(n, h, d)
}

fn new_drop(n: &'static str, d: &'static str, i: ItemType) -> Item {
    Item::new(n, d, i)
}

impl Place {
    pub fn new(location: Location) -> Self {
        use Location as Loc;
        match location {
            Loc::Forest => Self::build(location, "Ooh mystical"),
            Loc::Mountains => Self::build(location, "The goats bite..."),
            Loc::Cave => Self::build(location, "Spooky"),
        }
    }

    fn build(location: Location, desc: &'static str) -> Self {
        Self {
            name: location.to_string(),
            location,
            desc: desc.to_string(),
        }
    }

    pub fn get_registered() -> Vec<Location> {
        vec![Location::Forest, Location::Mountains, Location::Cave]
    }

    pub fn get_entities(&self) -> Vec<(Entity, f64)> {
        use Location as Loc;
        match self.location {
            Loc::Forest => vec![
                // Name, Max HP, Attack Range (Min, Max), Encounter % (e.g., 0.01 = 1%)
                (new_ent("Goblin", 75, (5, 15)), 0.5),
                (new_ent("Elf", 100, (10, 20)), 0.45),
                (new_ent("Mud Wizard", 120, (20, 45)), 0.05),
            ],
            Loc::Mountains => vec![
                (new_ent("Goat", 95, (15, 25)), 0.5),
                (new_ent("Snowman", 110, (20, 35)), 0.45),
                (new_ent("Yeti", 175, (30, 55)), 0.05),
            ],
            Loc::Cave => vec![
                (new_ent("Spider", 110, (20, 30)), 0.5),
                (new_ent("Dweller", 125, (25, 45)), 0.45),
                (new_ent("Stalactite Golem", 200, (40, 65)), 0.05),
            ],
        }
    }

    /// Includes base drops
    pub fn get_drops(&self) -> Drops {
        use Location as Loc;
        // Base items
        let mut list = Item::get_base_drops();
        let exclusives = match self.location {
            Loc::Forest => vec![
                (
                    new_drop(
                        "Wooden Sword",
                        "Give your enemies splinters",
                        IType::Weapon { damage: 35 },
                    ),
                    0.25,
                    false,
                ),
                (
                    new_drop(
                        "Wooden Armor",
                        "May occasionally give you splinters",
                        ItemType::Armor { reduction: 0.25 },
                    ),
                    0.1,
                    false,
                ),
                (
                    new_drop("Mountains Key", "Unlocks the mountains", IType::Key),
                    0.05,
                    false,
                ),
            ],
            Loc::Mountains => vec![
                (
                    new_drop("Iron Sword", "Very pointy", IType::Weapon { damage: 50 }),
                    0.25,
                    false,
                ),
                (
                    new_drop(
                        "Iron Armor",
                        "Tough stuff",
                        ItemType::Armor { reduction: 0.40 },
                    ),
                    0.1,
                    false,
                ),
                (
                    new_drop("Cave Key", "Unlocks the cave", IType::Key),
                    0.05,
                    false,
                ),
            ],
            Loc::Cave => vec![
                (
                    new_drop("Blessed Sword", "Hallelujah", IType::Weapon { damage: 60 }),
                    0.25,
                    false,
                ),
                (
                    new_drop(
                        "Blessed Armor",
                        "Legends say an angel kissed this",
                        ItemType::Armor { reduction: 0.55 },
                    ),
                    0.15,
                    false,
                ),
            ],
        };
        list.extend(exclusives);
        list
    }
}
