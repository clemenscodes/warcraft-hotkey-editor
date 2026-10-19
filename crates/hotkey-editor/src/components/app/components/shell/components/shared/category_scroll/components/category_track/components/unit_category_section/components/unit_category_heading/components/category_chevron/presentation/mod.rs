use super::model::CategoryChevronModel;
use crate::components::app::components::shell::components::shared::icons::{
    ICON_CHEVRON_DOWN, ICON_CHEVRON_RIGHT,
};

pub(super) struct CategoryChevronPresentation {
    pub(super) icon: &'static str,
}

impl From<&CategoryChevronModel> for CategoryChevronPresentation {
    fn from(props: &CategoryChevronModel) -> Self {
        let icon = match props.is_collapsed {
            true => ICON_CHEVRON_RIGHT,
            false => ICON_CHEVRON_DOWN,
        };
        Self { icon }
    }
}

impl ddd::Layered for CategoryChevronPresentation {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Presentation for CategoryChevronPresentation {
    type Model = CategoryChevronModel;
}
