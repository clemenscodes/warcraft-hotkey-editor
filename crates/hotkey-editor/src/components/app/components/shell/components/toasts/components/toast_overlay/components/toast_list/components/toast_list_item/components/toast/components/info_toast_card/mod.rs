mod data;
mod model;
mod presentation;
mod view;

pub use view::InfoToastCardView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_overlay::components::toast_list::components::toast_list_item::components::toast::components::shared::toast_card::ToastCard;
use data::ICON;
use dioxus::prelude::*;
use model::InfoToastCardModel;
use presentation::InfoToastCardPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InfoToastCard(props: InfoToastCardModel) -> Element {
    let InfoToastCardPresentation {
        title,
        description,
        id,
    } = InfoToastCardPresentation::from(&props);
    let on_remove = props.on_remove;
    rsx! {
        div {
            class: CLASS,
            ToastCard {
                icon: ICON,
                title,
                description,
                id,
                on_remove,
            }
        }
    }
}

assert_component!(InfoToastCard);
