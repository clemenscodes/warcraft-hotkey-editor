use casclib::open;
use std::env;

fn main() {
    let casc_root = env::args().nth(1).expect("casc root");
    let pattern = env::args()
        .nth(2)
        .unwrap_or_else(|| "selecthero".to_string())
        .to_ascii_lowercase();

    let storage = open(&casc_root).expect("open CASC");
    let mut count = 0;
    for entry in storage.files_with_mask("*") {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let name = entry.get_name().to_ascii_lowercase();
        if name.contains(&pattern) {
            println!("{}", entry.get_name());
            count += 1;
            if count > 50 {
                break;
            }
        }
    }
}
