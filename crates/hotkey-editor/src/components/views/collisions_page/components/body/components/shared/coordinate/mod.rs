mod props;
mod style;
use crate::assert_component;
use dioxus::prelude::*;
pub use props::CoordinateProps;
use style::CLASS;
assert_component!(Coordinate);
/// A command-card coordinate: displays the column and row of the domain
/// `GridCoordinate` it is handed.
#[component]
pub fn Coordinate(props: CoordinateProps) -> Element {
    let coordinate = props.coordinate;
    let column = u8::from(coordinate.column());
    let row = u8::from(coordinate.row());
    rsx! {
        span {
            class: CLASS,
            "Column {column} Row {row}"
        }
    }
}
