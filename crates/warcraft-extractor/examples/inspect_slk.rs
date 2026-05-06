//! Read-only SLK column / row introspection for unit-form ability research.
//!
//! Usage (inside `nix develop`):
//!     cargo run -p warcraft-extractor --example inspect_slk -- \
//!         "$WARCRAFT_PATH" \
//!         units/unitabilities.slk \
//!         eden uobs ubsp
//!
//! The trailing positional args are unit / object ids; the example prints
//! every column header in the SLK and then dumps every row whose first cell
//! matches one of the requested ids (case-insensitive). Pass no ids to dump
//! only the column headers.

use casclib::open;
use warcraft_slk::SlkTable;

fn main() {
    let mut args = std::env::args().skip(1);
    let casc_root = args
        .next()
        .expect("expected first arg: path to Warcraft III CASC root");
    let slk_path_filter = args
        .next()
        .expect("expected second arg: SLK relative path (e.g. units/unitabilities.slk)");
    let row_id_filters: Vec<String> = args.map(|argument| argument.to_ascii_lowercase()).collect();

    let storage = open(&casc_root).expect("failed to open CASC storage");

    let lower_filter = slk_path_filter.to_ascii_lowercase();
    let mut merged_table = SlkTable::from("");
    let mut matched_paths: Vec<String> = Vec::new();
    for entry_result in storage.files::<String>() {
        let entry = entry_result.expect("CASC entry iter failed");
        let entry_name = entry.get_name().to_string();
        let normalized_name = entry_name.replace('\\', "/").to_ascii_lowercase();
        if !normalized_name.ends_with(&lower_filter) {
            continue;
        }

        let mut bytes = Vec::new();
        let handle = entry.open().expect("failed to open CASC entry");
        handle
            .extract(&mut bytes)
            .expect("failed to extract CASC bytes");
        let text = std::str::from_utf8(&bytes).expect("SLK was not UTF-8");
        let table = SlkTable::from(text);
        merged_table.extend([table]);
        matched_paths.push(entry_name);
    }

    if matched_paths.is_empty() {
        eprintln!("no SLK file in CASC matched filter `{slk_path_filter}`");
        std::process::exit(1);
    }

    println!("=== matched {} CASC entries ===", matched_paths.len());
    for path in &matched_paths {
        println!("  · {path}");
    }
    println!();

    let column_names = sorted_column_names(&merged_table);
    println!("merged columns ({}):", column_names.len());
    println!("{}", column_names.join(", "));

    if row_id_filters.is_empty() {
        return;
    }

    let id_column_candidates = [
        "unitAbilID",
        "alias",
        "unitID",
        "unitUIID",
        "unitBalanceID",
        "abilCode",
    ];
    let primary_column = id_column_candidates
        .iter()
        .find(|candidate| column_names.iter().any(|name| name == *candidate))
        .copied()
        .unwrap_or_else(|| column_names.first().map(String::as_str).unwrap_or(""));
    println!();
    println!("(using `{primary_column}` as primary id column)");

    let mut printed_any = false;
    for row_view in &merged_table {
        let raw_id = row_view.get(primary_column).unwrap_or("");
        let matches_filter = row_id_filters
            .iter()
            .any(|filter_id| filter_id.eq_ignore_ascii_case(raw_id));
        if !matches_filter {
            continue;
        }
        print_row(&column_names, &row_view);
        printed_any = true;
    }
    if !printed_any {
        let mut sample_ids: Vec<String> = (&merged_table)
            .into_iter()
            .filter_map(|row_view| row_view.get(primary_column).map(str::to_string))
            .filter(|id| !id.is_empty())
            .take(20)
            .collect();
        sample_ids.sort();
        eprintln!(
            "no rows matched the requested ids: {}",
            row_id_filters.join(", ")
        );
        eprintln!(
            "first {} ids in `{primary_column}`: {sample_ids:?}",
            sample_ids.len()
        );
    }
}

fn sorted_column_names(table: &SlkTable) -> Vec<String> {
    let mut names: Vec<String> = table
        .columns()
        .values()
        .filter(|name| !name.is_empty())
        .cloned()
        .collect();
    names.sort();
    names.dedup();
    names
}

fn print_row(column_names: &[String], row_view: &warcraft_slk::RowView<'_>) {
    let primary_id = column_names
        .iter()
        .find_map(|column_name| row_view.get(column_name))
        .unwrap_or("");
    println!();
    println!("--- row: {primary_id} ---");
    for column_name in column_names {
        let raw_value = row_view.get(column_name).unwrap_or("");
        if raw_value.is_empty() || raw_value == "_" || raw_value == "-" {
            continue;
        }
        println!("    {column_name} = {raw_value}");
    }
}
