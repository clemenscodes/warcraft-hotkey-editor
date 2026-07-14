mod model;
mod view;

pub use view::HelpSectionTitleView;
mod style;

use dioxus::prelude::*;
use model::HelpSectionTitleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HelpSectionTitle(props: HelpSectionTitleModel) -> Element {
    let title = props.title;
    rsx! {
        h3 {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(HelpSectionTitle);
