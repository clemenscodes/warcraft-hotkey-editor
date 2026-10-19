mod data;
mod model;
mod view;

pub use view::SearchDialogView;

use crate::components::app::components::shell::components::shared::search_dialog_body::SearchDialogBodyView;
use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use crate::services::search_dialog::context::use_search_dialog_dismiss_provider;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::SearchDialogModel;
use tw_macro::assert_component;

#[component]
pub fn SearchDialog(props: SearchDialogModel) -> Element {
    let open = props.open;
    let on_open_change = props.on_open_change;
    use_search_dialog_dismiss_provider(on_open_change);
    let body = SearchDialogBodyView::default();
    rsx! {
        WarcraftDialog::<SearchDialogBodyView, Empty> {
            title: data::TITLE,
            body,
            open,
            on_open_change,
        }
    }
}

assert_component!(SearchDialog);
