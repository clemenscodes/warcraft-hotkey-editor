use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::grid_layout_editor_dialog::data::INTRO_LINES;

pub(super) struct LayoutIntroLineView {
    pub(super) line: String,
}

pub(super) fn intro_lines() -> Vec<LayoutIntroLineView> {
    INTRO_LINES
        .iter()
        .map(|&line| {
            let line = line.to_string();
            LayoutIntroLineView { line }
        })
        .collect()
}
