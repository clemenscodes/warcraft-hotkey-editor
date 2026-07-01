mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::IslandCoordProps;
use style::CLASS;
assert_component!(IslandCoord);
/// A coordinate label on an island card.
#[component]
pub fn IslandCoord(props: IslandCoordProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
