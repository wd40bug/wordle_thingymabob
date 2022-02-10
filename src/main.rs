use wordle_thingymabob::graphics;
use wordle_thingymabob::file_mod;
fn main() {
    graphics::run();
    let mystery_word = file_mod::get_mystery_word().unwrap();
    wordle_thingymabob::run(&mystery_word);
}
