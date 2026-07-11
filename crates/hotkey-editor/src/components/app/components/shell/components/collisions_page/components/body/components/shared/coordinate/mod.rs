mod model;
mod view;

pub use view::CoordinateView;
mod style;
use dioxus::prelude::*;
use model::CoordinateModel;
use style::CLASS;
use tw_macro::assert_component;
/// A command-card coordinate: displays the column and row of the domain
/// `GridCoordinate` it is handed.
#[component]
pub fn Coordinate(props: CoordinateModel) -> Element {
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

assert_component!(Coordinate);
