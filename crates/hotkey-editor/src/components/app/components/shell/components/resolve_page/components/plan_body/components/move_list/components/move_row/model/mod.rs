use super::view::MoveRowView;
use crate::components::app::components::shell::components::resolve_page::presentation::MoveView;
use dioxus::prelude::*;

/// One planned move to render: its shaped view. The navigation its name links through is
/// read from context, so it is not a prop. Each ability icon owns and opens its own
/// carriers dialog.
#[derive(Props, Clone, PartialEq)]
pub struct MoveRowModel {
    pub move_view: MoveView,
}

impl From<&MoveRowView> for MoveRowModel {
    fn from(view: &MoveRowView) -> Self {
        let MoveRowView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Model for MoveRowModel {
    type View = MoveRowView;
}
