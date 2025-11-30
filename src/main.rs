mod keymap_parser;
use std::path::Path;

fn main() {
    // Parse keymap
    let keymap_path =
        Path::new("/home/apm/qmk_firmware/keyboards/ferris/keymaps/default/keymap.json");
    let km = keymap_parser::load_keymap(keymap_path).unwrap();
    println!("{:#?}", km);

    // Parse keycodes
    let keycode_dir = Path::new("/home/apm/qmk_firmware/data/constants/keycodes");
    match keymap_parser::parse_keycodes(keycode_dir) {
        Ok(keycodes) => {
            let parsed_keymap = keymap_parser::parse_keymap_layers(&km, &keycodes);
            println!("\n Parsed keymap:");
            println!("{:#?}", parsed_keymap);
            // println!("\nExample lookups:");
            // println!(
            //     "KC_A = {}",
            //     keymap_parser::get_keycode_label("KC_A", &keycodes)
            // );
            // println!(
            //     "KC_ESC = {}",
            //     keymap_parser::get_keycode_label("KC_ESC", &keycodes)
            // );
        }
        Err(e) => eprintln!("Error loading keycodes: {}", e),
    }
}
