use crate::components::app::components::shell::components::shared::icons::IconUrl;

/// The published `View` contract mirroring [`UnitCardIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct UnitCardIconView {
    pub icon_path: Option<IconUrl>,
    pub display_name: String,
}

impl ddd::View for UnitCardIconView {}
