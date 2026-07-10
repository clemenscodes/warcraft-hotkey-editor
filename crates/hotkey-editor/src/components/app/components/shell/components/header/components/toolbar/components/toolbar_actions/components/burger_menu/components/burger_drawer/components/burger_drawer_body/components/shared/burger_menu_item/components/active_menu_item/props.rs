use dioxus::prelude::*;

/// The active-look drawer row's props: the icon markup and label text it composes,
/// plus the accessibility/e2e attributes and click handler the `<button>` needs.
/// The dispatcher names these fields from `BurgerMenuItemProps`.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveMenuItemProps {
    pub icon: &'static str,
    pub label: String,
    pub disabled: bool,
    pub role: Option<&'static str>,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<&'static str>,
    pub aria_pressed: Option<&'static str>,
    pub aria_label: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
}
