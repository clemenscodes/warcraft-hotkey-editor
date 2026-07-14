use super::view::BurgerDrawerBodyView;
use dioxus::prelude::*;

/// The drawer body's only input: the drawer's close handler, passed through to the file-action
/// menu's non-dialog rows that dismiss the drawer.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerBodyModel {
    pub on_close: EventHandler<MouseEvent>,
}

impl From<&BurgerDrawerBodyView> for BurgerDrawerBodyModel {
    fn from(view: &BurgerDrawerBodyView) -> Self {
        let BurgerDrawerBodyView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Model for BurgerDrawerBodyModel {
    type View = BurgerDrawerBodyView;
}
