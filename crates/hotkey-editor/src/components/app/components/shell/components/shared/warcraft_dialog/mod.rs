pub mod components;
mod frame;
mod model;
mod style;
mod view;

pub use view::WarcraftDialogView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::body_scroll_lock::use_body_scroll_lock;
use browser_kit::frame::Render;
use components::warcraft_dialog_header::WarcraftDialogHeaderView;
use dioxus::prelude::*;
use dioxus_kit::frame::Dialog;
use frame::WarcraftDialogFrame;
use model::WarcraftDialogModel;
use style::CLASS;
use tw_macro::assert_component;

/// The reusable app dialog: the styled content box + title/close header chrome over the
/// headless `Dialog` frame primitive. The caller supplies only a body region, a title, and
/// the open state; this owns the header — built from the close it derives from
/// `on_open_change` — and its single `CLASS` (the content box), which the headless `Dialog`
/// applies to its content container via `class:`. It never sees the document. Every app
/// dialog reuses it by composition.
#[component]
pub fn WarcraftDialog<Body: Render<Output = Element>, Footer: Render<Output = Element>>(
    props: WarcraftDialogModel<Body, Footer>,
) -> Element {
    let title = props.title;
    let body = props.body;
    let footer = props.footer;
    let open = props.open;
    let on_open_change = props.on_open_change;
    use_body_scroll_lock();
    let on_close: Callback<()> = Callback::new(move |()| on_open_change.call(false));
    let header = WarcraftDialogHeaderView { title, on_close };
    let frame = WarcraftDialogFrame {
        header,
        body,
        footer,
    };
    rsx! {
        Dialog {
            frame,
            open,
            on_open_change,
            class: CLASS,
        }
    }
}

assert_component!(WarcraftDialog);
