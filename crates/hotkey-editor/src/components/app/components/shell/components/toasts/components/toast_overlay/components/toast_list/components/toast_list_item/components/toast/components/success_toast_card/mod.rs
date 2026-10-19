mod data;
mod model;
mod presentation;
mod view;

pub use view::SuccessToastCardView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_overlay::components::toast_list::components::toast_list_item::components::toast::components::shared::toast_card::ToastCard;
use data::ICON;
use dioxus::prelude::*;
use model::SuccessToastCardModel;
use presentation::SuccessToastCardPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SuccessToastCard(props: SuccessToastCardModel) -> Element {
    let SuccessToastCardPresentation {
        title,
        description,
        id,
    } = SuccessToastCardPresentation::from(&props);
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

assert_component!(SuccessToastCard);
