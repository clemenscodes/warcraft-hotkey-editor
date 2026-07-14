use super::model::UnitCardIconModel;
use crate::components::app::components::shell::components::shared::framed_icon::IconRadius;

/// The framed portrait's presentation: the image source, alt text, and the fixed
/// `FramedIcon` axes this slot draws with. Built purely from the model — a shaping
/// leaf, no effects.
pub struct UnitCardIconPresentation {
    pub(super) src: Option<String>,
    pub(super) alt: String,
    pub(super) radius: IconRadius,
    pub(super) hover_glow: bool,
    pub(super) placeholder: bool,
}

impl From<&UnitCardIconModel> for UnitCardIconPresentation {
    fn from(model: &UnitCardIconModel) -> Self {
        let UnitCardIconModel {
            icon_path,
            display_name,
        } = model.clone();
        let src = icon_path.map(|icon_url| icon_url.to_string());
        let radius = IconRadius::Hairline;
        let hover_glow = false;
        let placeholder = true;
        Self {
            src,
            alt: display_name,
            radius,
            hover_glow,
            placeholder,
        }
    }
}

impl ddd::Presentation for UnitCardIconPresentation {
    type Model = UnitCardIconModel;
}
