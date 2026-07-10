use super::state::BurgerItemState;
use super::view::BurgerMenuItemView;
use dioxus::prelude::*;

/// A single drawer row. Its label and icon are content; its visual weight is the
/// `state`; its behaviour is the click handler; and the accessibility/e2e
/// attributes it needs are threaded as optional fields (only the ones a given
/// row carries are set, the rest stay `None` and are omitted from the markup).
#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuItemProps {
    pub icon: &'static str,
    pub label: String,
    pub state: BurgerItemState,
    pub disabled: bool,
    pub role: Option<&'static str>,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<&'static str>,
    pub aria_pressed: Option<&'static str>,
    pub aria_label: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BurgerMenuItemView> for BurgerMenuItemProps {
    fn from(view: &BurgerMenuItemView) -> Self {
        let BurgerMenuItemView {
            icon,
            label,
            state,
            disabled,
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            onclick,
        } = view.clone();
        Self {
            icon,
            label,
            state,
            disabled,
            role,
            aria_haspopup,
            aria_expanded,
            aria_pressed,
            aria_label,
            onclick,
        }
    }
}

impl ddd::Props for BurgerMenuItemProps {
    type View = BurgerMenuItemView;
}
