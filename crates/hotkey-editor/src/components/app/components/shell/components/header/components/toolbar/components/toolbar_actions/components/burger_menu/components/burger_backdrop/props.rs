use super::view::BurgerBackdropView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerBackdropProps {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BurgerBackdropView> for BurgerBackdropProps {
    fn from(view: &BurgerBackdropView) -> Self {
        let BurgerBackdropView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Props for BurgerBackdropProps {
    type View = BurgerBackdropView;
}
