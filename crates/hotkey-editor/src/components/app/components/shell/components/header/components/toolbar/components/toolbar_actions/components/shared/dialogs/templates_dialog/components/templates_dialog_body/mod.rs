pub mod components;
mod model;
mod view;

pub use view::TemplatesDialogBodyView;
mod style;

use crate::services::viewport::use_is_pager_viewport;
use components::template_gallery::TemplateGallery;
use components::template_pager::TemplatePager;
use dioxus::prelude::*;
use model::TemplatesDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TemplatesDialogBody(props: TemplatesDialogBodyModel) -> Element {
    let cards = props.cards;
    let is_pager = use_is_pager_viewport();
    if is_pager {
        return rsx! {
            div {
                class: CLASS,
                TemplatePager {
                    cards,
                }
            }
        };
    }
    rsx! {
        div {
            class: CLASS,
            TemplateGallery {
                cards,
            }
        }
    }
}

assert_component!(TemplatesDialogBody);
