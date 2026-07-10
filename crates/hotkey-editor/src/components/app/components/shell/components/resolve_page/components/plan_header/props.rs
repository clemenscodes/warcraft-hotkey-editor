use dioxus::prelude::*;

/// The plan header: the move/unresolved summary and the Apply button.
#[derive(Props, Clone, PartialEq)]
pub struct PlanHeaderProps {
    #[props(into)]
    pub moves_text: String,
    pub unresolved_count: usize,
    pub running: bool,
    pub on_apply: EventHandler<MouseEvent>,
}
