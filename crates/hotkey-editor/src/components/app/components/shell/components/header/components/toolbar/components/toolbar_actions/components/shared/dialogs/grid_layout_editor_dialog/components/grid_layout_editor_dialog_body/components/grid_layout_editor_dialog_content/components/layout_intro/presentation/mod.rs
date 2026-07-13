use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::grid_layout_editor_dialog::data::INTRO_LINES;

/// One instruction line as plain data, threaded to the intro block, which renders
/// a [`LayoutIntroLine`](super::components::layout_intro_line::LayoutIntroLine) per line.
pub(super) struct LayoutIntroLineView {
    pub(super) line: String,
}

/// The intro copy as finished lines, one per entry in the intro data.
pub(super) fn intro_lines() -> Vec<LayoutIntroLineView> {
    INTRO_LINES
        .iter()
        .map(|&line| {
            let line = line.to_string();
            LayoutIntroLineView { line }
        })
        .collect()
}
