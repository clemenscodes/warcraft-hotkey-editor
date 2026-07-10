use warcraft_keybinds::GridCoordinate;

/// The published `View` contract mirroring [`CoordinateProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CoordinateView {
    pub coordinate: GridCoordinate,
}

impl ddd::View for CoordinateView {}
