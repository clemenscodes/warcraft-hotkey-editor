use dioxus::prelude::*;

/// The invisible full-screen backdrop that closes the menu when clicked outside.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelBackdropProps {
    pub onclick: EventHandler<MouseEvent>,
}
