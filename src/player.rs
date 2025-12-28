use crate::inform;
use crate::warn;

use super::entities::Entity;
use super::items::*;
use super::places::Place;
use super::utilities::*;

// Testing
pub type Inventory = Vec<(Item, u16)>; // Item and quantity

pub struct Player {
    pub name: String,
    pub cur_place: Place,
    pub max_health: u16,
    pub cur_health: u16,
    pub armor: Option<Item>,
    pub xp: f64,
    pub xp_multiplier: f64,
    pub level: u16,
    pub inventory: Inventory,
    pub weapon: Item,
}

impl Player {
    pub fn new(name: String) -> Self {
        let starter_weapon: Item = Item::new("Fists", "Punchy", ItemType::Weapon { damage: 25 });
        Player {
            name,
            cur_place: Place::new(crate::places::Location::Forest),
            max_health: 100,
            cur_health: 100,
            armor: None,
            xp: 0.,
            xp_multiplier: 1.,
            level: 1,
            inventory: Vec::new(),
            weapon: starter_weapon,
        }
    }
    pub fn take_damage(&mut self, amount: u16, penetrating: bool) {
        let proc_damage = match &self.armor {
            Some(arm) if !penetrating => {
                let ItemType::Armor { reduction } = arm.item_type else {
                    return eprintln!("Equipped 'armor' isn't of type `IType::Armor`");
                };
                (amount as f64 * (1. - reduction)).round() as u16
            }
            _ => amount,
        };
        self.cur_health -= proc_damage.min(self.cur_health);
    }

    pub fn heal(&mut self, amount: u16) -> u16 {
        let proc = amount.min(self.max_health - self.cur_health);
        self.cur_health += proc;
        proc // Return amount healed for display
    }

    pub fn attack(&self, entity: &mut Entity) {
        if let ItemType::Weapon { damage, .. } = self.weapon.item_type {
            entity.cur_health -= damage.min(entity.cur_health);
        } else {
            eprintln!("Player doesn't have a weapon equipped... Negligence");
        }
    }

    pub fn is_alive(&self) -> bool {
        self.cur_health != 0
    }

    pub fn is_full_hp(&self) -> bool {
        self.cur_health == self.max_health
    }

    pub fn update_xp(&mut self, for_defeating: &Entity) {
        // (& level up)
        let xp_gain = math::calc_xp_gain(for_defeating) * self.xp_multiplier;
        self.xp += xp_gain;

        let lvl_change = math::calc_level(self.xp) - self.level;
        inform!(
            "You defeated the {} and earned {}{} XP{}!",
            for_defeating.name,
            color("Cyan"),
            xp_gain as u32,
            color("Blue")
        );

        if lvl_change > 0 {
            for _ in 0..lvl_change {
                self.raise_level();
            }
        }
    }

    fn raise_level(&mut self) {
        self.level += 1;
        inform!(
            "\n{}You reached level {}!{}",
            style("Italics"),
            paint_text(&self.level.to_string(), "Cyan"),
            style("Reset")
        );
        // Award the player an item every 5 levels
        if self.level % 5 == 0 {
            let base_drops = Item::get_base_drops();
            self.fetch_drop(base_drops, "& were awarded a(n)");
        }
    }
    pub fn display_leveling(&self) {
        let Self { level, xp, .. } = *self;
        let cur_lvl_xp = math::xp_needed(level) as u16;
        let next_lvl_xp = math::xp_needed(level + 1) as u16;

        let cur_prog = (xp as u16).saturating_sub(cur_lvl_xp);
        let for_next_lvl = next_lvl_xp - cur_lvl_xp;

        inform!("\t \nNeeded for next level:\n");
        println!(
            "|{}| {} |{}| [{}]\n",
            level,
            progress_bar(cur_prog, for_next_lvl, "Cyan", 15),
            level + 1,
            format!("{}/{} XP", xp as u16, next_lvl_xp)
        );
    }

    pub fn display_health(&self) {
        let Self {
            cur_health,
            max_health,
            ..
        } = *self;
        if cur_health <= 30 {
            warn!("!!! Critical health !!!\n")
        }
        println!(
            " {}\n{}",
            format!("{}/{} HP", cur_health, max_health),
            progress_bar(cur_health, max_health, "Green", 10)
        );
    }

    pub fn fetch_drop(&mut self, pool: Drops, msg: &str) {
        let Some(chosen) = Item::roll_drop(pool, self) else {
            return eprintln!("Fetching drop failed");
        };
        inform!(
            "{msg} {}{}{}! It can be found in your inventory",
            color("Cyan"),
            chosen.name,
            color("Blue")
        );
        self.add_to_inventory(chosen, 1);
    }

    fn equip_weapon(&mut self, weapon: Item) {
        self.weapon = weapon;
    }

    fn equip_armor(&mut self, armor: Item) {
        self.armor = Some(armor);
    }

    pub fn remove_from_inventory(&mut self, location: usize, amount: u16) {
        if let Some((_, qty)) = self.inventory.get_mut(location) {
            *qty = qty.saturating_sub(amount);
            if *qty == 0 {
                self.inventory.remove(location);
            }
        }
    }

    pub fn equip_from_inventory(&mut self, location: usize) {
        let Some((item, _)) = self.inventory.get(location).cloned() else {
            return eprintln!("Item doesn't exist in inventory");
        };
        match item.item_type {
            ItemType::Weapon { .. } => {
                let equipped_weapon = self.weapon.clone();
                self.add_to_inventory(equipped_weapon, 1);
                self.equip_weapon(item);
            }
            ItemType::Armor { .. } => {
                let equipped_armor = self.armor.take();
                if let Some(arm) = equipped_armor {
                    self.add_to_inventory(arm, 1);
                }
                self.equip_armor(item);
            }
            except => return eprintln!("Attempted to equip a(n) `{except}` item"),
        }
        self.remove_from_inventory(location, 1);
    }

    pub fn add_to_inventory(&mut self, item: Item, quant: u16) {
        if let Some((_, qty)) = self.inventory.iter_mut().find(|(i, _)| *i == item) {
            *qty += quant;
        } else {
            self.inventory.push((item, quant));
        }
    }

    pub fn has_item(&self, name: &str) -> bool {
        self.inventory.iter().any(|(i, _)| i.name == name)
            || self.weapon.name == name
            || self.armor.as_ref().is_some_and(|i| i.name == name)
    }

    pub fn silly(&mut self) {
        let stuff = [
            // Item::new("Apple", "Crunchy :3", ItemType::Healer {amount: 25}),
            // Item::new("Sword", "Pointy", ItemType::Weapon {damage: 45}),
            Item::new("Holy Blade", "Hallelujah", ItemType::Weapon { damage: 999 }),
            Item::new(
                "Divine Armor",
                "Hallelujah 2",
                ItemType::Armor { reduction: 0.88 },
            ),
            Item::new("Mountains Key", "..", ItemType::Key),
            Item::new("Cave Key", "...", ItemType::Key),
            Item::new("Tranquility Stone", "Meditate instantly", ItemType::Key),
        ];

        for itm in stuff {
            self.add_to_inventory(itm, 69);
        }
    }
}
