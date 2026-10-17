pub mod components;
mod model;
mod view;

pub use view::TemplatesDialogBodyView;
mod style;

use crate::services::viewport::use_is_touch_viewport;
use components::template_gallery::TemplateGallery;
use components::template_pager::TemplatePager;
use dioxus::prelude::*;
use model::TemplatesDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TemplatesDialogBody(props: TemplatesDialogBodyModel) -> Element {
    let cards = props.cards;
    let is_touch = use_is_touch_viewport();
    if is_touch {
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
