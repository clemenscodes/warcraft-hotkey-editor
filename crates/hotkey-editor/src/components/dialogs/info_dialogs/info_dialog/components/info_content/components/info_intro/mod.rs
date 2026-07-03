mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::InfoIntroProps;
use style::CLASS;
assert_component!(InfoIntro);

/// Every info dialog's lead-in line, its copy handed in as a prop.
#[component]
pub fn InfoIntro(props: InfoIntroProps) -> Element {
    let intro = props.intro;
    rsx! {
        p { class: CLASS, "{intro}" }
    }
}
