use super::view::MoveRowView;
use crate::components::app::components::shell::components::resolve_page::logic::MoveView;
use dioxus::prelude::*;

/// One planned move to render: its shaped view. The navigation its name links through is
/// read from context, so it is not a prop. Each ability icon owns and opens its own
/// carriers dialog.
#[derive(Props, Clone, PartialEq)]
pub struct MoveRowProps {
    pub move_view: MoveView,
}

impl From<&MoveRowView> for MoveRowProps {
    fn from(view: &MoveRowView) -> Self {
        let MoveRowView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Props for MoveRowProps {
    type View = MoveRowView;
}
