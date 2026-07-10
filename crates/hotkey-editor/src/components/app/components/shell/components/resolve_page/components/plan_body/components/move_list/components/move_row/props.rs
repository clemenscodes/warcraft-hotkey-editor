use crate::components::app::components::shell::components::resolve_page::logic::MoveView;
use dioxus::prelude::*;

/// One planned move to render: its shaped view. The navigation its name links through is
/// read from context, so it is not a prop. Each ability icon owns and opens its own
/// carriers dialog.
#[derive(Props, Clone, PartialEq)]
pub struct MoveRowProps {
    pub move_view: MoveView,
}
