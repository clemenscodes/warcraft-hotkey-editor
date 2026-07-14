use super::view::BurgerDrawerView;
use dioxus::prelude::*;

/// The slide-in drawer's only input: the close handler, threaded from the controller and passed
/// to the header's close control and to the non-dialog rows that dismiss the drawer.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerModel {
    pub on_close: EventHandler<MouseEvent>,
}

impl From<&BurgerDrawerView> for BurgerDrawerModel {
    fn from(view: &BurgerDrawerView) -> Self {
        let BurgerDrawerView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Model for BurgerDrawerModel {
    type View = BurgerDrawerView;
}
