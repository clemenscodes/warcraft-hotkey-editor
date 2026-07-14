mod model;
mod view;

pub use view::MutedStatGainView;
mod style;

use dioxus::prelude::*;
use model::MutedStatGainModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MutedStatGain(props: MutedStatGainModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(MutedStatGain);
