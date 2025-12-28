use std::fs::File;
use std::{fmt::Display, io::Read};
use termion::*;

use crate::warn;

pub mod macros;
pub mod math;

pub fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

/// I love you indexize <3
pub fn indexize(s: &str, range: usize) -> Option<usize> {
    match s.parse::<usize>() {
        Ok(num) if (1..=range).contains(&num) => Some(num - 1),
        _ => {
            warn!("Invalid input, please try again");
            None
        }
    }
}
/// Lists items of an iterative type, assuming they implement `Display`:
///
/// [[1]] >> ToDisplay
///
/// [[2]] >> ToDisplay
///
/// [[n]] >> ToDisplay
pub fn list_items<T>(opts: T)
where
    T: IntoIterator,
    T::Item: Display,
{
    for (i, opt) in opts.into_iter().enumerate() {
        let list_num = i + 1;
        println!(
            "{}[{list_num}] >> {}{}{}",
            color("Cyan"),
            color("LightMagenta"),
            opt,
            color("Reset")
        );
    }
}

pub fn progress_bar(current: u16, max: u16, clr: &str, segs: u16) -> String {
    let to_eq = max / segs;
    let curr_eq = (current / to_eq).min(segs) as usize;
    let remaining = (segs as usize) - curr_eq;

    format!(
        "{}[{}{}{}{0}]{}",
        color("Magenta"),
        color(clr),
        "=".repeat(curr_eq),
        " ".repeat(remaining),
        color("Reset")
    )
}

pub fn color(s: &str) -> Box<dyn std::fmt::Display> {
    match s {
        "Black" => Box::new(color::Fg(color::Black)),
        "Blue" => Box::new(color::Fg(color::Blue)),
        "Cyan" => Box::new(color::Fg(color::Cyan)),
        "Green" => Box::new(color::Fg(color::Green)),
        "LightGreen" => Box::new(color::Fg(color::LightGreen)),
        "LightMagenta" => Box::new(color::Fg(color::LightMagenta)),
        "Magenta" => Box::new(color::Fg(color::Magenta)),
        "Red" => Box::new(color::Fg(color::Red)),
        "Reset" => Box::new(color::Fg(color::Reset)),
        _ => panic!("Color `{}` is not registered", s),
    }
}

pub fn paint_text<T: Display>(s: T, clr: &str) -> String {
    format!("{}{}{}", color(clr), s, color("Reset"))
}

pub fn style(s: &str) -> &'static dyn std::fmt::Display {
    match s {
        "Bold" => &style::Bold,
        "Italics" => &style::Italic,
        "Faint" => &style::Faint,
        "Reset" => &style::Reset,
        _ => panic!("Style `{}` is not registered", s),
    }
}

pub fn show_sprite(path: String) {
    let operation = File::open(
        format!("src/sprites/{}", path), // e.g., places/forest.ans
    );

    let mut content = match operation {
        Ok(file) => file,
        Err(er) => return eprintln!("Loading sprite failed: `{er}`"),
    };

    let mut buffer = String::new();
    let loader = content.read_to_string(&mut buffer);

    match loader {
        Ok(_) => (),
        Err(er) => return eprintln!("Loading sprite failed: `{er}`"),
    };

    println!("{buffer}");
}
