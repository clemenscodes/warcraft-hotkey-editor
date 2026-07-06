use super::color_codes::WarcraftColorCodes;
use super::level_markers::LevelMarkers;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Tip;

impl Tip {
    pub fn lines_from(raw_tip: &str) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for raw_segment in raw_tip.split(['\n', ',']) {
            let level_markers_stripped = LevelMarkers::stripped(raw_segment);
            let trimmed_segment = level_markers_stripped
                .trim()
                .trim_end_matches(|character: char| character == '-' || character.is_whitespace())
                .trim()
                .to_string();
            if trimmed_segment.is_empty() {
                continue;
            }
            if let Some(last_segment) = output.last()
                && last_segment == &trimmed_segment
            {
                continue;
            }
            output.push(trimmed_segment);
        }
        output
    }

    pub fn shortened(raw_tip: &str) -> String {
        let color_stripped_tip = WarcraftColorCodes::stripped(raw_tip);
        let trimmed_tip = color_stripped_tip.trim();
        let after_hotkey_hint = match trimmed_tip.strip_prefix('(') {
            Some(rest_after_open) => match rest_after_open.find(')') {
                Some(close_index) => rest_after_open[close_index + 1..].trim_start(),
                None => trimmed_tip,
            },
            None => trimmed_tip,
        };
        let first_segment = after_hotkey_hint
            .split([',', '['])
            .next()
            .unwrap_or(after_hotkey_hint);
        first_segment.trim().to_string()
    }
}
