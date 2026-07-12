pub mod components;
mod model;
mod view;

pub use view::InfoToastCardView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_overlay::components::toast_list::components::toast_list_item::components::toast::components::shared::toast_close::ToastClose;
use components::info_toast_content::InfoToastContent;
use components::info_toast_icon::InfoToastIcon;
use dioxus::prelude::*;
use model::InfoToastCardModel;
use style::CLASS;
use tw_macro::assert_component;

/// The info toast card: its tinted surface owning the alertdialog root, the
/// info icon and title, the description, and the close control. Presentational
/// only; the dispatcher builds its props from the toast record.
#[component]
pub fn InfoToastCard(props: InfoToastCardModel) -> Element {
    let title = props.record.title().to_string();
    let description = props.record.description();
    let id = props.record.id();
    let on_remove = props.on_remove;
    rsx! {
        div {
            class: CLASS,
            role: "alertdialog",
            "aria-modal": "false",
            tabindex: "0",
            InfoToastIcon {}
            InfoToastContent { title, description }
            ToastClose { id, on_remove }
        }
    }
}

assert_component!(InfoToastCard);
