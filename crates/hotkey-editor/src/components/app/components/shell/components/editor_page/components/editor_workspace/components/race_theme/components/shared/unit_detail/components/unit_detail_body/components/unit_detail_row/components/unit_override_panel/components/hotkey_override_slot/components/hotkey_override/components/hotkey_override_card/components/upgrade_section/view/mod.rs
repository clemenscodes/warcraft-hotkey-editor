use dioxus::prelude::*;

/// The published `View` contract mirroring [`UpgradeSectionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UpgradeSectionView {
    /// Whether this ability has an upgraded form to show controls for.
    pub show: bool,
    pub upgrade_hotkey_label: String,
    pub upgrade_is_editing: bool,
    pub upgrade_hotkey_is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl ddd::View for UpgradeSectionView {}
