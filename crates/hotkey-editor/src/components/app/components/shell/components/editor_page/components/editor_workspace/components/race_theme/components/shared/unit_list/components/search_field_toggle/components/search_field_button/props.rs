use dioxus::prelude::*;

/// One search-field option: its label, whether it is the active field, and the
/// handler that selects it.
#[derive(Props, Clone, PartialEq)]
pub struct SearchFieldButtonProps {
    pub label: &'static str,
    pub is_active: bool,
    pub on_select: EventHandler<MouseEvent>,
}
