pub mod components;
mod model;
mod view;

pub use view::WarcraftDialogHeaderView;

use components::dialog_header::DialogHeader;
use dioxus::prelude::*;
use model::WarcraftDialogHeaderModel;
use tw_macro::assert_component;

#[component]
pub fn WarcraftDialogHeader(props: WarcraftDialogHeaderModel) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let handle_close = EventHandler::new(move |()| on_close.call(()));
    rsx! {
        DialogHeader {
            title,
            on_close: handle_close,
        }
    }
}

assert_component!(WarcraftDialogHeader);
