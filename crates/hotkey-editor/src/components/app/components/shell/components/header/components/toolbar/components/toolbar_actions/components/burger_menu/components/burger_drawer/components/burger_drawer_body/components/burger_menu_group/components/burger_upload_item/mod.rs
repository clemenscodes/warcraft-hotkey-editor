mod data;
mod model;
mod presentation;
mod style;
mod view;

pub use view::BurgerUploadItemView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerMenuItem;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::UploadInfoDialog;
use dioxus::prelude::*;
use model::BurgerUploadItemModel;
use presentation::{BurgerUploadItemPresentation, use_burger_upload_item};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerUploadItem(props: BurgerUploadItemModel) -> Element {
    let BurgerUploadItemPresentation {
        icon,
        label,
        state,
        role,
        open,
        onclick,
        on_open_change,
    } = use_burger_upload_item(&props);
    rsx! {
        div {
            class: CLASS,
            BurgerMenuItem {
                icon,
                label,
                state,
                disabled: false,
                role,
                aria_haspopup: None,
                aria_expanded: None,
                aria_pressed: None,
                aria_label: None,
                onclick,
            }
            UploadInfoDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerUploadItem);
