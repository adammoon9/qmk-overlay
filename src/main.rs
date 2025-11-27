mod keymap_parser;
use keymap_parser::qmk_json;

fn main() {
    let keycodes = qmk_json::parse_keycodes();

    println!("{:#?}", keycodes);
    // qmk_json::parse_keymap();
}
