#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LevelMarkers;

impl LevelMarkers {
    pub fn stripped(input: &str) -> String {
        let level_marker_prefix = "[Level ";
        let mut output = String::with_capacity(input.len());
        let mut remainder = input;
        while let Some(start_offset) = remainder.find(level_marker_prefix) {
            let prefix_slice = &remainder[..start_offset];
            let after_marker = &remainder[start_offset..];
            let close_offset_option = after_marker.find(']');
            let Some(close_offset) = close_offset_option else {
                output.push_str(prefix_slice);
                output.push_str(after_marker);
                return output;
            };
            let inner_slice = &after_marker[level_marker_prefix.len()..close_offset];
            let inner_is_numeric = !inner_slice.is_empty()
                && inner_slice
                    .chars()
                    .all(|inner_character| inner_character.is_ascii_digit());
            if !inner_is_numeric {
                let advance_end = start_offset + 1;
                output.push_str(&remainder[..advance_end]);
                remainder = &remainder[advance_end..];
                continue;
            }
            output.push_str(prefix_slice);
            remainder = &after_marker[close_offset + 1..];
        }
        output.push_str(remainder);
        output
    }
}
