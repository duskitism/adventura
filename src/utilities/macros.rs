#[macro_export]
macro_rules! warn {
    ($($arg:expr),*) => {
        println!("{}{}{}{}{}", color("Red"), style("Italics"), format!($($arg),*), color("Reset"), style("Reset"))
    };
}

#[macro_export]
macro_rules! inform {
    ($($arg:expr),*) => {
        println!("{}{}{}", color("Blue"), format!($($arg),*), color("Reset"))
    };
}

#[macro_export]
macro_rules! prompt {
    ($($arg:expr),*) => {
        input!("\n{}{}{}", color("Blue"), format!($($arg),*), color("Reset"))
            .trim()
            .to_string()
    };
}
