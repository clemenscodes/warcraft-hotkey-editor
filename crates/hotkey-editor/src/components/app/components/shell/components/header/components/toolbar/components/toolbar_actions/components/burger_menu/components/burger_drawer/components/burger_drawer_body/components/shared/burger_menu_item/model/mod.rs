use super::state::BurgerItemState;
use super::view::BurgerMenuItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuItemModel {
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

impl From<&BurgerMenuItemView> for BurgerMenuItemModel {
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

impl ddd::Model for BurgerMenuItemModel {
    type View = BurgerMenuItemView;
}
