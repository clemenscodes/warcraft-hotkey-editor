pub mod components;
mod props;
mod view;

pub use view::WarningToastCardView;
mod style;

use crate::components::app::components::shell::components::toasts::components::toast_container::components::toast_list::components::toast_list_item::components::toast_card::components::shared::toast_close::ToastClose;
use components::warning_toast_content::WarningToastContent;
use components::warning_toast_icon::WarningToastIcon;
use dioxus::prelude::*;
use props::WarningToastCardProps;
use style::CLASS;
use tw_macro::assert_component;

/// The warning toast card: its tinted surface owning the alertdialog root, the
/// warning icon and title, the description, and the close control. Presentational
/// only; the dispatcher builds its props from the toast record.
#[component]
pub fn WarningToastCard(props: WarningToastCardProps) -> Element {
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
            WarningToastIcon {}
            WarningToastContent { title, description }
            ToastClose { id, on_remove }
        }
    }
}

assert_component!(WarningToastCard);
