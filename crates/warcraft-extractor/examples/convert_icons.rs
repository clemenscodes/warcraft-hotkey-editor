use std::path::PathBuf;

use image::{ImageBuffer, Rgba};
use warcraft_extractor::{BlpDecoder, DdsDecoder};

/// Decode one or more command-button textures into the editor's PNG icon
/// format using the same proven decoders the extraction pipeline uses: DDS
/// for the HD set, BLP for the classic textures some icons still ship in.
///
/// Usage: convert_icons <output_dir> <dds_or_blp_path>...
/// Each output is written to <output_dir>/<lowercased file stem>.png.
fn main() {
    let mut arguments = std::env::args().skip(1);
    let output_dir_arg = arguments
        .next()
        .expect("usage: convert_icons <output_dir> <dds_or_blp_path>...");
    let output_dir = PathBuf::from(output_dir_arg);
    for texture_argument in arguments {
        let texture_path = PathBuf::from(&texture_argument);
        let texture_bytes = std::fs::read(&texture_path).expect("read texture bytes");
        let is_blp = texture_path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("blp"));
        let decoded_image = if is_blp {
            BlpDecoder::decode(&texture_bytes).expect("decode blp")
        } else {
            DdsDecoder::decode(&texture_bytes).expect("decode dds")
        };
        let width = decoded_image.width();
        let height = decoded_image.height();
        let rgba = decoded_image.rgba();
        let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba)
            .expect("rgba buffer from decoded image");
        let file_stem = texture_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("texture file stem");
        let lowercased_stem = file_stem.to_ascii_lowercase();
        let output_name = format!("{lowercased_stem}.png");
        let output_path = output_dir.join(output_name);
        buffer.save(&output_path).expect("save png");
        let output_display = output_path.display();
        println!("wrote {output_display} ({width}x{height})");
    }
}
