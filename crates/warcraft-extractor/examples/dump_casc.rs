use std::env;
use std::path::PathBuf;

use tracing_subscriber::EnvFilter;
use warcraft_extractor::{ExtractionPipeline, GAME_EXTRACTION_RULE};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .with_writer(std::io::stderr)
        .init();

    let casc_root = env::args()
        .nth(1)
        .expect("usage: dump_casc <casc-root> <dest-dir>");
    let dest_dir_string = env::args()
        .nth(2)
        .expect("usage: dump_casc <casc-root> <dest-dir>");
    let dest_dir = PathBuf::from(dest_dir_string);

    if let Err(error) =
        ExtractionPipeline::run(&casc_root, Some(&dest_dir), &[GAME_EXTRACTION_RULE])
    {
        eprintln!("error: {error}");
        eprintln!("  casc root: {casc_root}");
        eprintln!("  dest dir:  {}", dest_dir.display());
        std::process::exit(1);
    }
}
