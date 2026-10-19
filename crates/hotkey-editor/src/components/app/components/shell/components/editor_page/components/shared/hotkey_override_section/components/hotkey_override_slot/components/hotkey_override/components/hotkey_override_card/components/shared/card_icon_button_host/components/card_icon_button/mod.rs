mod model;
mod view;

pub use view::CardIconButtonView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::CardIconButtonModel;

#[component]
pub fn CardIconButton(props: CardIconButtonModel) -> Element {
    let icon = props.icon;
    let title = props.title;
    let aria_label = props.aria_label;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            title,
            aria_label,
            onclick: move |_| on_click.call(()),
            dangerous_inner_html: icon,
        }
    }
}

assert_component!(CardIconButton);
