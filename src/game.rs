use std::fmt::Display;
use std::{thread, time::Duration};

use crate::inform;
use crate::prompt;
use crate::warn;

use super::places::*;
use super::player::*;

use Location as Loc;

use super::items::*;
use ItemType as IType;

use super::utilities::*;

#[derive(Clone, Copy)]
enum Options {
    Explore,
    Travel,
    Meditate,
    ViewInventory,
    ViewStats,
}

impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Explore => write!(f, "Explore"),
            Self::Travel => write!(f, "Travel"),
            Self::Meditate => write!(f, "Meditate"),
            Self::ViewInventory => write!(f, "View Inventory"),
            Self::ViewStats => write!(f, "View Stats"),
        }
    }
}
impl Options {
    pub fn get_registered() -> Vec<Self> {
        vec![
            Self::Explore,
            Self::Travel,
            Self::Meditate,
            Self::ViewInventory,
            Self::ViewStats,
        ]
    }
}

pub fn run_game() {
    let mut plr: Player = setup_plr();
    // plr.silly(); // Add junk to inventory

    while plr.is_alive() {
        let plr_option: Options = get_option();
        carry_option(plr_option, &mut plr);
    }
    warn!("You died...");
}

pub fn replay() -> bool {
    loop {
        let inp = prompt!("\nWould you like to play again? Enter (y/n): ");
        match inp.to_lowercase().as_str() {
            "y" => {
                break {
                    clear_terminal();
                    true
                };
            }
            "n" => break false,
            _ => warn!("Invalid input"),
        }
    }
}

fn setup_plr() -> Player {
    println!(
        r"{}    _      _             _                 
   /_\  __| |_ _____ _ _| |_ _  _ _ _ __ _ 
  / _ \/ _` \ V / -_) ' \  _| || | '_/ _` |
 /_/ \_\__,_|\_/\___|_||_\__|\_,_|_| \__,_|                             
{}",
        color("LightGreen"),
        color("Reset")
    );

    let plr_name: String = 'O: loop {
        let inp = prompt!("Please enter your name: ");
        const PROHIBITED_NAMES: &[&str] = &["god", "devil", "gabriel"];

        if inp.is_empty() || PROHIBITED_NAMES.iter().any(|p| inp.eq_ignore_ascii_case(p)) {
            warn!("Invalid name, please try again");
            continue;
        }

        loop {
            let confirm = prompt!("Is '{inp}' okay? Enter (y/n): ");
            match confirm.to_lowercase().as_str() {
                "y" => break 'O inp,
                "n" => continue 'O,
                _ => warn!("Invalid input"),
            }
        }
    };

    println!(
        "\nWelcome, {}{}{plr_name}{}{}!\n",
        color("Cyan"),
        style("Bold"),
        color("Reset"),
        style("Reset")
    );
    prompt!("Press `enter` to continue ");

    Player::new(plr_name)
}

fn get_option() -> Options {
    clear_terminal();
    inform!("What would you like to do?\n");

    let options = Options::get_registered();
    let len = options.len();
    list_items(&options);

    loop {
        let inp: String = prompt!("Type a matching number (1-{len}): ");
        let Some(ind) = indexize(&inp, len) else {
            continue;
        };

        break options[ind];
    }
}

fn carry_option(opt: Options, plr: &mut Player) {
    match opt {
        Options::Explore => explore(plr),
        Options::Travel => travel(plr),
        Options::Meditate => meditate(plr),
        Options::ViewInventory => view_inventory(plr),
        Options::ViewStats => view_stats(plr),
    }
}

// Freedom of will
static GREEN: fn(&str) -> String = |s: &str| {
    format!(
        "{}{}{}",
        style("Bold"),
        paint_text(s, "Green"),
        style("Reset")
    )
};

fn show_header(c: &str) {
    println!("{}", GREEN(c))
}

fn explore(plr: &mut Player) {
    clear_terminal();
    let p_name = plr.cur_place.name.to_lowercase();

    show_header(&format!("You're exploring the {p_name}\n"));
    show_sprite(format!("places/{}.ans", p_name));

    prompt!("Press `enter` to continue ");

    let chest_found = math::bool_from_chance(0.1);
    if chest_found {
        chest_logic(plr)
    } else {
        entity_logic(plr)
    }
}

fn chest_logic(plr: &mut Player) {
    clear_terminal();

    let cur_place = &plr.cur_place;
    show_header(&format!(
        "You found a chest in the {}!",
        cur_place.name.to_lowercase()
    ));
    show_sprite(String::from("misc/chest.ans"));

    let reward_pool = cur_place.get_drops();
    prompt!("Press `enter` to open\n");
    plr.fetch_drop(reward_pool, "You found a(n)");
    prompt!("Press `enter` to continue ");
}

fn entity_logic(plr: &mut Player) {
    let places_entities = plr.cur_place.get_entities();
    let Some(mut chosen_entity) = math::weigh_vec(places_entities) else {
        return eprintln!("Failed to fetch an entity");
    };
    chosen_entity.encounter(plr);
}

fn travel(plr: &mut Player) {
    clear_terminal();

    println!("{}Travel Options: {}\n", color("Blue"), color("Reset"));
    let locations = Place::get_registered();
    let travel_opts = travel_opts(&locations);
    let len = locations.len();
    list_items(travel_opts); // ...Also dump the Vec

    loop {
        let inp = prompt!(
            "\nWhere would you like to go? Type a matching number (1-{}) or 'enter' to exit: ",
            len
        );
        if inp.is_empty() {
            return;
        }

        let Some(ind) = indexize(&inp, len) else {
            continue;
        };
        let loc = locations[ind]; // Indexize assures this
        let key_name = format!("{} Key", loc);

        if !loc.req_key() || plr.has_item(&key_name) {
            plr.cur_place = Place::new(loc);
            prompt!(
                "You're now at the {}. Press `enter` to continue ",
                loc.to_string().to_lowercase()
            );
            break;
        } else {
            warn!("You don't have access to this place")
        }
    }
}

fn travel_opts(locs: &[Loc]) -> Vec<String> {
    locs.into_iter()
        .map(|loc| {
            if loc.req_key() {
                format!(
                    "{}{} (Requires `{0} Key`){}",
                    loc,
                    color("Green"),
                    color("Reset")
                )
            } else {
                loc.to_string()
            }
        })
        .collect()
}

fn meditate(plr: &mut Player) {
    clear_terminal();
    show_header("You're rejuvenating...");
    show_sprite(String::from("misc/buddha.ans"));

    if !plr.has_item("Tranquility Stone") {
        thread::sleep(Duration::from_millis(2_500));
    }

    let heal_amount = math::rng_from_range::<u16>((20, 60));
    let actual = plr.heal(heal_amount);

    if plr.is_full_hp() {
        inform!("You're fully healed:\n")
    } else {
        inform!(
            "You healed {}{actual}{} HP after deep contemplation:\n",
            color("Cyan"),
            color("Blue")
        );
    }
    plr.display_health();
    prompt!("Press `enter` to continue ");
}

pub fn view_inventory(plr: &mut Player) {
    loop {
        clear_terminal();
        display_inv_items(&plr.inventory);

        if plr.inventory.is_empty() {
            println!("Inventory is empty 🫙");
            show_equipped(plr);
            return drop(prompt!("\nPress `enter` to exit "));
        }

        show_equipped(plr);

        let len: usize = plr.inventory.len();
        let inp = prompt!(
            "Type a matching number (1-{len}) to view an item or `enter` to exit inventory: "
        );
        let Some(selected) = indexize(&inp, len) else {
            return;
        };

        manage_item(plr, selected);
    }
}

fn show_equipped(plr: &Player) {
    let armor = match &plr.armor {
        Some(i) => &i.name,
        _ => "None equipped",
    };

    println!(
        "\n{} {}\n{} {}",
        GREEN("Equipped Weapon:"),
        plr.weapon.name,
        GREEN("Equipped Armor:"),
        armor,
    );
}

fn display_inv_items(inventory: &Inventory) {
    println!(
        "{}Here's your inventory: {}\n",
        color("Blue"),
        color("Reset")
    );

    let proc: Vec<_> = inventory
        .into_iter()
        .map(|(itm, qty)| format!("{} (x{})", itm.name, qty))
        .collect();

    list_items(&proc);
}

fn info_f(f: &str, v: impl Display) -> String {
    format!("{} {}", paint_text(f, "Magenta"), v)
}

fn manage_item(plr: &mut Player, selected: usize) {
    const OPTS: &[&str] = &["Use", "Equip", "Delete"];
    clear_terminal();

    if let Some((itm, qty)) = plr.inventory.get(selected) {
        let itm = itm.clone();
        let itm_type = itm.item_type;

        let item_info = [
            Some(info_f("Name:", &itm.name)),
            Some(info_f("Quantity:", qty)),
            Some(info_f("Description:", &itm.desc)),
            Some(info_f("Class:", &itm_type)),
            match itm_type {
                IType::Weapon { damage } => Some(info_f("Damage:", damage)),
                IType::Healer { amount } => Some(info_f("Heals:", amount)),
                IType::Armor { reduction } => Some(info_f(
                    "Damage Reduction:",
                    format!("{:.1}%", reduction * 100.),
                )),
                _ => None,
            },
        ];

        inform!("\nItem Information:\n");
        for field in item_info.into_iter().flatten() {
            println!("{field}")
        }

        let options = match itm_type {
            IType::Weapon { .. } | IType::Armor { .. } => vec![OPTS[1], OPTS[2]],
            IType::Healer { .. } | IType::Special { .. } => vec![OPTS[0], OPTS[2]],
            IType::Key => return drop(prompt!("Press `enter` to go back ")),
        };

        let len = options.len();

        inform!("\nHere are your options:\n");
        list_items(&options);

        let chosen = loop {
            let inp = prompt!(
                "Type a matching number (1-{len}) to manage this item or `enter` to go back: "
            );

            if inp.is_empty() {
                return;
            }
            if let Some(val) = indexize(&inp, len) {
                break val;
            }
        };

        match options.get(chosen) {
            Some(&"Use") => itm.use_item(plr, selected),
            Some(&"Equip") => plr.equip_from_inventory(selected),
            Some(&"Delete") => loop {
                warn!("\nAre you sure you want to delete this item?");
                let inp = prompt!("All copies will be deleted 🤯 Enter (y/n): ");
                match inp.to_lowercase().as_str() {
                    "y" => break plr.remove_from_inventory(selected, *qty),
                    "n" => break,
                    _ => warn!("Invalid input"),
                }
            },
            _ => {}
        }
    }
}

fn view_stats(plr: &Player) {
    let Player {
        level,
        xp_multiplier,
        ..
    } = *plr;
    clear_terminal();
    inform!("Here are your stats:");

    inform!("\n\n--- Health: ---\n");
    plr.display_health();
    inform!("\n\n--- Leveling: ---\n");
    println!(
        "{} {level}\n{} x{xp_multiplier:.2}",
        GREEN("Current Level:"),
        GREEN("XP Multiplier:")
    );
    plr.display_leveling();

    prompt!("Press `enter` to exit ");
}
