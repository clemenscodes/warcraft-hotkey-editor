use crate::components::app::components::shell::components::shared::icons::IconUrl;

#[derive(Clone, PartialEq)]
pub struct UnitCardIconView {
    pub icon_path: Option<IconUrl>,
    pub display_name: String,
}

impl ddd::View for UnitCardIconView {}
