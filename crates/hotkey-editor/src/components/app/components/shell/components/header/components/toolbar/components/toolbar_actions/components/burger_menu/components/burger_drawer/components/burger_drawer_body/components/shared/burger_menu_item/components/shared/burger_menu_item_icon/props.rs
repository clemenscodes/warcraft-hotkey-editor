use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuItemIconProps {
    pub svg: &'static str,
}
