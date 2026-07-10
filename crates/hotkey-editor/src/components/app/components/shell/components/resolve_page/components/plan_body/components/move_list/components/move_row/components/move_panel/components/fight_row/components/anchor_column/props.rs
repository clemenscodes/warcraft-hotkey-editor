use super::view::AnchorColumnView;
use crate::components::app::components::shell::components::resolve_page::logic::MoveView;
use dioxus::prelude::*;

/// The optional rival column of a move card: the move whose rival ability it renders as
/// a name plate over an icon. The column renders nothing when the move has no rival.
#[derive(Props, Clone, PartialEq)]
pub struct AnchorColumnProps {
    pub move_view: MoveView,
}

impl From<&AnchorColumnView> for AnchorColumnProps {
    fn from(view: &AnchorColumnView) -> Self {
        let AnchorColumnView { move_view } = view.clone();
        Self { move_view }
    }
}

impl ddd::Props for AnchorColumnProps {
    type View = AnchorColumnView;
}
