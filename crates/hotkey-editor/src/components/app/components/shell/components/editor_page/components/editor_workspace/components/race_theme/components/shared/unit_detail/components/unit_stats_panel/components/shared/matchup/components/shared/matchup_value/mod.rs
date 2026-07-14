mod model;
mod view;

pub use view::MatchupValueView;
mod style;

use dioxus::prelude::*;
use model::MatchupValueModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MatchupValue(props: MatchupValueModel) -> Element {
    let percent = props.multiplier * 100.0;
    rsx! {
        span {
            class: CLASS,
            "{percent:.0}%"
        }
    }
}

assert_component!(MatchupValue);
