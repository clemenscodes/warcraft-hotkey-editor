mod data;
mod model;
mod presentation;
mod view;

pub use view::WarningToastCardView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_overlay::components::toast_list::components::toast_list_item::components::toast::components::shared::toast_card::ToastCard;
use data::ICON;
use dioxus::prelude::*;
use model::WarningToastCardModel;
use presentation::WarningToastCardPresentation;
use style::CLASS;
use tw_macro::assert_component;

/// The warning toast: a thin wrapper that publishes the warning tint as CSS custom
/// properties and renders the shared [`ToastCard`]. Its own root is `contents`, so it
/// adds no box — only the colour vars the card's descendants read.
#[component]
pub fn WarningToastCard(props: WarningToastCardModel) -> Element {
    let WarningToastCardPresentation {
        title,
        description,
        id,
    } = WarningToastCardPresentation::from(&props);
    let on_remove = props.on_remove;
    rsx! {
        div {
            class: CLASS,
            ToastCard { icon: ICON, title, description, id, on_remove }
        }
    }
}

assert_component!(WarningToastCard);
