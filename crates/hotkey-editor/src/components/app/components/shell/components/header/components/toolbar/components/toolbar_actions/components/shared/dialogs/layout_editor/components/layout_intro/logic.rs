use super::super::super::data::INTRO_LINES;
use super::components::layout_intro_line::LayoutIntroLineProps;

/// The intro copy as finished line props, one per entry in the intro data.
pub(super) fn intro_lines() -> Vec<LayoutIntroLineProps> {
    INTRO_LINES
        .iter()
        .map(|&line| {
            let line = line.to_string();
            LayoutIntroLineProps { line }
        })
        .collect()
}
