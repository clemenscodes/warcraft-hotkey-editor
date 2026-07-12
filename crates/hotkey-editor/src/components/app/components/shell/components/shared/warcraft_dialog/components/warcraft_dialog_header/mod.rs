mod model;
mod view;

pub use view::WarcraftDialogHeaderView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeader;
use dioxus::prelude::*;
use model::WarcraftDialogHeaderModel;
use tw_macro::assert_component;

/// The dialog's fixed title bar chrome — the ONE shared dialog header. It composes the
/// shared `DialogHeader` (mirrored gold decorations, the title, the close control),
/// adapting the dialog's `Callback` close to the `EventHandler` the header leaf takes.
/// Every WarcraftDialog renders exactly this; individual dialogs never duplicate the
/// chrome.
#[component]
pub fn WarcraftDialogHeader(props: WarcraftDialogHeaderModel) -> Element {
    let title = props.title;
    let on_close = props.on_close;
    let handle_close = EventHandler::new(move |()| on_close.call(()));
    rsx! {
        DialogHeader { title, on_close: handle_close }
    }
}

assert_component!(WarcraftDialogHeader);
