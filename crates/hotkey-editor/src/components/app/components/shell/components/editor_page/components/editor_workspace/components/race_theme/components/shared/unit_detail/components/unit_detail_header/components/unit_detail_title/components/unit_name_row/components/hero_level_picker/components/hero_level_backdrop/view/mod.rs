use dioxus::prelude::*;

/// The published `View` contract mirroring [`HeroLevelBackdropModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HeroLevelBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for HeroLevelBackdropView {}
