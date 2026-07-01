use dioxus::prelude::*;

/// The dragged slot's key label, shown on the follower.
#[derive(Props, Clone, PartialEq)]
pub struct InventoryDragKeyProps {
    #[props(into)]
    pub label: String,
}
