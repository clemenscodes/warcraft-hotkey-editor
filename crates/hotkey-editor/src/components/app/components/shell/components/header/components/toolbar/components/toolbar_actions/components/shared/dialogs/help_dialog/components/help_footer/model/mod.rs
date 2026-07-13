use super::view::HelpFooterView;

/// The footer host's ddd model. The component sources the dismiss wiring from context in its
/// presentation builder, so this is the fieldless published contract the frame's footer
/// region names as its `Render::Model`.
#[derive(Clone, PartialEq, Default)]
pub struct HelpFooterModel;

impl From<&HelpFooterView> for HelpFooterModel {
    fn from(_view: &HelpFooterView) -> Self {
        Self
    }
}

impl ddd::Model for HelpFooterModel {
    type View = HelpFooterView;
}
