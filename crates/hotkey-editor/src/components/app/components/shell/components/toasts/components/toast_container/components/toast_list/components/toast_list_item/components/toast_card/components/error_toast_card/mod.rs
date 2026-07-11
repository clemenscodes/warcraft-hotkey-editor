pub mod components;
mod model;
mod view;

pub use view::ErrorToastCardView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastClose;
use components::error_toast_content::ErrorToastContent;
use components::error_toast_icon::ErrorToastIcon;
use dioxus::prelude::*;
use model::ErrorToastCardModel;
use style::CLASS;
use tw_macro::assert_component;

/// The error toast card: its tinted surface owning the alertdialog root, the
/// error icon and title, the description, and the close control. Presentational
/// only; the dispatcher builds its props from the toast record.
#[component]
pub fn ErrorToastCard(props: ErrorToastCardModel) -> Element {
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
            ErrorToastIcon {}
            ErrorToastContent { title, description }
            ToastClose { id, on_remove }
        }
    }
}

assert_component!(ErrorToastCard);
