use std::{collections::BTreeMap, path::PathBuf};

use casclib::open;
use warcraft_database::{WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS};
use warcraft_extractor::DefaultCustomKeys;

const LOCALIZED_STRINGS_PREFIX: &str = "war3.w3mod:_locales/enus.w3mod:units/";
const SHIPPED_CUSTOM_KEYS: &str = "war3.w3mod:customkeys.txt";

/// Generate the editor's default `CustomKeys.txt` from a Warcraft III CASC
/// archive: the game's default hotkeys (localized `*strings.txt`, overlaid by
/// Blizzard's shipped `customkeys.txt`) plus every default grid position from
/// the game database. Regenerate `db.rs` from the same archive first.
///
/// Usage: default_customkeys <casc_root> <output_path>...
fn main() {
    let mut arguments = std::env::args().skip(1);
    let casc_root = arguments
        .next()
        .expect("usage: default_customkeys <casc_root> <output_path>...");
    let output_paths: Vec<PathBuf> = arguments.map(PathBuf::from).collect();
    let storage = open(&casc_root).expect("open CASC storage");

    let mut localized_strings: BTreeMap<String, String> = BTreeMap::new();
    let mut shipped_custom_keys: Option<String> = None;
    for entry_result in storage.files_with_mask("*") {
        let Ok(entry) = entry_result else {
            continue;
        };
        let entry_name = entry.get_name().to_ascii_lowercase().replace('\\', "/");
        let is_localized_strings =
            entry_name.starts_with(LOCALIZED_STRINGS_PREFIX) && entry_name.ends_with("strings.txt");
        let is_shipped_custom_keys = entry_name == SHIPPED_CUSTOM_KEYS;
        if !is_localized_strings && !is_shipped_custom_keys {
            continue;
        }
        let mut bytes = Vec::new();
        let _ = entry.open().expect("open CASC entry").extract(&mut bytes);
        let text = String::from_utf8_lossy(&bytes).into_owned();
        if is_shipped_custom_keys {
            shipped_custom_keys = Some(text);
        } else {
            localized_strings.insert(entry_name, text);
        }
    }
    let mut default_custom_keys = DefaultCustomKeys::default();
    for text in localized_strings.values() {
        default_custom_keys.overlay(text);
    }
    let shipped_text = shipped_custom_keys.expect("customkeys.txt present in CASC");
    default_custom_keys.overlay(&shipped_text);

    let rendered = default_custom_keys.render(&WARCRAFT_DATABASE, WARCRAFT_SYSTEM_KEYBINDS);
    for output_path in output_paths {
        std::fs::write(&output_path, &rendered).expect("write default CustomKeys.txt");
        let output_display = output_path.display();
        let source_count = localized_strings.len();
        println!("wrote {output_display} from {source_count} strings files + customkeys.txt");
    }
}
