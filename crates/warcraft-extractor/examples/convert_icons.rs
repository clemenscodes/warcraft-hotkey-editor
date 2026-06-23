use std::path::PathBuf;

use image::{ImageBuffer, Rgba};
use warcraft_extractor::DdsDecoder;

/// Decode one or more DDS command-button textures into the editor's PNG icon
/// format using the same proven decoder the extraction pipeline uses.
///
/// Usage: convert_icons <output_dir> <dds_path>...
/// Each output is written to <output_dir>/<lowercased file stem>.png.
fn main() {
    let mut arguments = std::env::args().skip(1);
    let output_dir_arg = arguments
        .next()
        .expect("usage: convert_icons <output_dir> <dds_path>...");
    let output_dir = PathBuf::from(output_dir_arg);
    for dds_argument in arguments {
        let dds_path = PathBuf::from(&dds_argument);
        let dds_bytes = std::fs::read(&dds_path).expect("read dds bytes");
        let decoded_image = DdsDecoder::decode(&dds_bytes).expect("decode dds");
        let width = decoded_image.width();
        let height = decoded_image.height();
        let rgba = decoded_image.rgba();
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba)
            .expect("rgba buffer from decoded image");
        let file_stem = dds_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("dds file stem");
        let lowercased_stem = file_stem.to_ascii_lowercase();
        let output_name = format!("{lowercased_stem}.png");
        let output_path = output_dir.join(output_name);
        buffer.save(&output_path).expect("save png");
        let output_display = output_path.display();
        println!("wrote {output_display} ({width}x{height})");
    }
}
