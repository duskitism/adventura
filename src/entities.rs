use input_macro::input;

use crate::prompt;
use crate::warn;

// CMD K + W
use super::game::view_inventory;
use super::player::*;
use super::utilities::math::rng_from_range;
use super::utilities::*;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Entity {
    pub name: String,
    pub max_health: u16,
    pub cur_health: u16,
    pub damage: (u16, u16),
}

impl Entity {
    pub fn new(name: &'static str, max_health: u16, damage: (u16, u16)) -> Self {
        Self {
            name: name.to_owned(),
            max_health,
            cur_health: max_health,
            damage,
        }
    }

    pub fn encounter(&mut self, plr: &mut Player) {
        encounter(self, plr);
    }

    fn get_damage(&self) -> u16 {
        rng_from_range(self.damage)
    }

    fn attack(&self, plr: &mut Player) {
        plr.take_damage(self.get_damage(), false);
    }

    fn is_alive(&self) -> bool {
        self.cur_health != 0
    }
}

fn ent_sprite(name: &str, plr: &Player) {
    let ent = name.replace(" ", "_").to_lowercase();
    let place = plr.cur_place.name.to_lowercase();

    show_sprite(format!("entities/{place}_entities/{ent}.ans"));
}

fn panel_builder(_: (), cur_hp: u16, max_hp: u16, name: &str, tabs: usize) {
    let t = "\t".repeat(tabs);
    println!(
        "
    {t}{}{}{} 
    {t}{}
    {t}\t{}/{} HP
    ",
        style("Bold"),
        name,
        style("Reset"),
        progress_bar(cur_hp, max_hp, "Green", 10),
        cur_hp,
        max_hp
    );
}

fn display_fight(entity: &Entity, plr: &Player) {
    panel_builder(
        clear_terminal(),
        entity.cur_health,
        entity.max_health,
        &entity.name,
        0,
    );

    panel_builder(
        ent_sprite(&entity.name, plr),
        plr.cur_health,
        plr.max_health,
        &plr.name,
        7,
    );
}

fn encounter(entity: &mut Entity, plr: &mut Player) {
    let options = ["Attack", "Inventory", "Flee"];
    let dropped_item = math::bool_from_chance(0.1);

    'MainLoop: while entity.is_alive() && plr.is_alive() {
        display_fight(entity, plr);
        list_items(&options);

        'InputLoop: loop {
            let input: String = prompt!("What would you like to do? Enter (1-{}): ", options.len());
            match input.as_str() {
                "1" => break 'InputLoop plr.attack(entity),
                "2" => {
                    view_inventory(plr);
                    continue 'MainLoop;
                }
                "3" => return drop(prompt!("You fled. Press `enter` to continue ")),
                _ => warn!("Invalid input, try again"),
            }
        } // Attack the player if the entity is still alive:
        if entity.is_alive() {
            entity.attack(plr);
        }
    }
    display_fight(entity, plr);

    if !plr.is_alive() {
        return;
    }
    plr.update_xp(entity);

    if dropped_item {
        let msg = format!("\nThe {} also dropped a(n)", entity.name);
        plr.fetch_drop(plr.cur_place.get_drops(), &msg);
    }
    plr.display_leveling();
    prompt!("\nPress `enter` to continue ");
}
