use dioxus::prelude::*;

/// The published `View` contract mirroring [`TileOverrideTierButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct TileOverrideTierButtonView {
    pub aria_label: &'static str,
    pub icon: &'static str,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::View for TileOverrideTierButtonView {}
