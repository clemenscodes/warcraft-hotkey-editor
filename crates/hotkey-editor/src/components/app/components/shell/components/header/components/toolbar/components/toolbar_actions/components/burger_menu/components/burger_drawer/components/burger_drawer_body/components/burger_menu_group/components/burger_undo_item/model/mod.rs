use super::view::BurgerUndoItemView;
use dioxus::prelude::*;

/// The undo drawer row's props: the drawer's close handler, threaded from the drawer body so
/// tapping the row runs the undo command and then closes the drawer.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerUndoItemModel {
    pub on_close: EventHandler<MouseEvent>,
}

impl From<&BurgerUndoItemView> for BurgerUndoItemModel {
    fn from(view: &BurgerUndoItemView) -> Self {
        let BurgerUndoItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Model for BurgerUndoItemModel {
    type View = BurgerUndoItemView;
}
