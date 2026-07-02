mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CoordinateProps;
use style::CLASS;
assert_component!(Coordinate);
/// A coordinate label on an island card.
#[component]
pub fn Coordinate(props: CoordinateProps) -> Element {
    let text = props.text;
    rsx! { span { class: CLASS, {text} } }
}
