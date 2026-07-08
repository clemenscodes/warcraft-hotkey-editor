use dioxus::prelude::*;
/// The "+N more" link opening the carriers dialog for an ability carried by more
/// units than the one shown on the card.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictMoreProps {
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}
