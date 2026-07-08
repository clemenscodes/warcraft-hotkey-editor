use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_icon::BurgerMenuItemIconProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::components::shared::burger_menu_item_label::BurgerMenuItemLabelProps;
use dioxus::prelude::*;

/// The idle-look drawer row's props: the already-shaped icon and label child props,
/// plus the accessibility/e2e attributes and click handler the `<button>` needs.
/// Built by the dispatcher from `BurgerMenuItemProps`; carrying the child props as
/// data is passing data, not `Element`.
#[derive(Props, Clone, PartialEq)]
pub struct IdleMenuItemProps {
    pub icon: BurgerMenuItemIconProps,
    pub label: BurgerMenuItemLabelProps,
    pub disabled: bool,
    pub role: Option<&'static str>,
    pub data_action: Option<&'static str>,
    pub aria_haspopup: Option<&'static str>,
    pub aria_expanded: Option<&'static str>,
    pub aria_pressed: Option<&'static str>,
    pub aria_label: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
}
