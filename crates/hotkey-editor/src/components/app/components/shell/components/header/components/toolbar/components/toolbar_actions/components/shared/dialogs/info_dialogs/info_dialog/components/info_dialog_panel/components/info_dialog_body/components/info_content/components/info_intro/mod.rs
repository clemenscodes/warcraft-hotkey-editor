mod props;
mod style;

use dioxus::prelude::*;
pub use props::InfoIntroProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(InfoIntro);

/// Every info dialog's lead-in line, its copy handed in as a prop.
#[component]
pub fn InfoIntro(props: InfoIntroProps) -> Element {
    let intro = props.intro;
    rsx! {
        p { class: CLASS, "{intro}" }
    }
}
