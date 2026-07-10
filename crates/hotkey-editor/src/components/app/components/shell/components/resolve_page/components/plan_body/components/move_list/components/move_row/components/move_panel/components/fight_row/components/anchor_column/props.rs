use crate::components::app::components::shell::components::resolve_page::logic::MoveView;
use dioxus::prelude::*;

/// The optional rival column of a move card: the move whose rival ability it renders as
/// a name plate over an icon. The column renders nothing when the move has no rival.
#[derive(Props, Clone, PartialEq)]
pub struct AnchorColumnProps {
    pub move_view: MoveView,
}
