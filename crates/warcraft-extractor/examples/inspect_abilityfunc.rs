use casclib::open;
use std::env;
use std::io::Cursor;

fn main() {
    let casc_root = env::args().nth(1).expect("casc root");
    let target_file = env::args().nth(2).expect("e.g. nightelfabilityfunc.txt");
    let id_filter: Vec<String> = env::args().skip(3).map(|s| s.to_lowercase()).collect();

    let storage = open(&casc_root).expect("open CASC");
    for entry in storage.files_with_mask("*") {
        let entry = entry.expect("file entry");
        let name = entry.get_name();
        let lower = name.to_ascii_lowercase();
        if !lower.ends_with(&target_file.to_ascii_lowercase()) {
            continue;
        }
        eprintln!("=== {} ===", name);
        let handle = match entry.open() {
            Ok(h) => h,
            Err(_) => continue,
        };
        let mut bytes = Vec::new();
        if handle.extract(Cursor::new(&mut bytes)).is_err() {
            continue;
        }
        let text = String::from_utf8_lossy(&bytes);
        let mut current_id: Option<String> = None;
        let mut buffer = Vec::<String>::new();
        let mut should_print = false;
        let flush = |id: &Option<String>, buf: &Vec<String>, print: bool| {
            if print {
                if let Some(id) = id {
                    eprintln!("\n--- [{}] ---", id);
                }
                for line in buf {
                    eprintln!("{}", line);
                }
            }
        };
        for raw_line in text.lines() {
            let line = raw_line.trim_end_matches('\r').to_string();
            if line.starts_with('[') && line.ends_with(']') {
                flush(&current_id, &buffer, should_print);
                buffer.clear();
                let id = line
                    .trim_matches(|c: char| c == '[' || c == ']')
                    .to_string();
                let id_lower = id.to_ascii_lowercase();
                should_print = id_filter.is_empty() || id_filter.iter().any(|f| f == &id_lower);
                current_id = Some(id);
            } else {
                buffer.push(line);
            }
        }
        flush(&current_id, &buffer, should_print);
    }
}
