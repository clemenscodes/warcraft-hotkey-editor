use dioxus::prelude::*;

use super::super::burger_drawer_body::BurgerDrawerBodyProps;
use super::super::burger_drawer_header::BurgerDrawerHeaderProps;
use super::super::burger_menu_item::BurgerMenuItemProps;

/// The slide-in drawer: everything the drawer subtree needs, threaded from the
/// controller — the close handler, the primary Grid Layout row, and the grouped
/// file-action rows.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerProps {
    pub on_close: EventHandler<MouseEvent>,
    pub layout: BurgerMenuItemProps,
    pub items: Vec<BurgerMenuItemProps>,
}

impl From<&BurgerDrawerProps> for BurgerDrawerHeaderProps {
    fn from(props: &BurgerDrawerProps) -> Self {
        let onclick = props.on_close;
        Self { onclick }
    }
}

impl From<&BurgerDrawerProps> for BurgerDrawerBodyProps {
    fn from(props: &BurgerDrawerProps) -> Self {
        let layout = props.layout.clone();
        let items = props.items.clone();
        Self { layout, items }
    }
}
