pub mod components;
mod model;
mod view;

pub use view::CardIconButtonHostView;
mod style;

use components::card_icon_button::CardIconButton;
use dioxus::prelude::*;
use model::CardIconButtonHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CardIconButtonHost(props: CardIconButtonHostModel) -> Element {
    let icon = props.icon;
    let title = props.title;
    let aria_label = props.aria_label;
    let on_click = props.on_click;
    rsx! {
        div {
            class: CLASS,
            CardIconButton {
                icon,
                title,
                aria_label,
                on_click,
            }
        }
    }
}

assert_component!(CardIconButtonHost);
