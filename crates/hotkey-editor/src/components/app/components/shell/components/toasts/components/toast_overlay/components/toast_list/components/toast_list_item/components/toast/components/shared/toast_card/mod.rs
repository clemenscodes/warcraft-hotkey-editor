pub mod components;
mod model;
mod view;

pub use view::ToastCardView;
mod style;

use components::toast_close::ToastClose;
use components::toast_content::ToastContent;
use components::toast_icon::ToastIcon;
use dioxus::prelude::*;
use model::ToastCardModel;
use style::CLASS;
use tw_macro::assert_component;

/// The shared toast surface: the tinted card owning the alertdialog root, the icon,
/// the text column, and the close control. The severity axis arrives entirely as CSS
/// custom properties (`--toast-accent`, `--toast-title`, `--glow-color`) set by the
/// per-severity wrapper that renders this card, so the look lives here once and every
/// severity reuses it by composition.
#[component]
pub fn ToastCard(props: ToastCardModel) -> Element {
    let icon = props.icon;
    let title = props.title;
    let description = props.description;
    let id = props.id;
    let on_remove = props.on_remove;
    rsx! {
        div {
            class: CLASS,
            role: "alertdialog",
            "aria-modal": "false",
            tabindex: "0",
            ToastIcon {
                icon,
            }
            ToastContent {
                title,
                description,
            }
            ToastClose {
                id,
                on_remove,
            }
        }
    }
}

assert_component!(ToastCard);
