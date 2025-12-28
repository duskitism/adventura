use adventura::game;

fn main() {
    game::run_game();
    while game::replay() {
        game::run_game();
    }
    println!("Program terminated");
}
