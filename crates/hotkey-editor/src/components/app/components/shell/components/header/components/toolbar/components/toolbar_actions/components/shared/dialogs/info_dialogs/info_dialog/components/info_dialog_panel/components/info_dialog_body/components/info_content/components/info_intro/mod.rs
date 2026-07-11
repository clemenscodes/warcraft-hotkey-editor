mod model;
mod view;

pub use view::InfoIntroView;
mod style;

use dioxus::prelude::*;
use model::InfoIntroModel;
use style::CLASS;
use tw_macro::assert_component;

/// Every info dialog's lead-in line, its copy handed in as a prop.
#[component]
pub fn InfoIntro(props: InfoIntroModel) -> Element {
    let intro = props.intro;
    rsx! {
        p { class: CLASS, "{intro}" }
    }
}

assert_component!(InfoIntro);
