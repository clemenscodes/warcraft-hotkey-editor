use super::view::HelpFooterHostView;

/// The footer host's ddd model. The component sources the dismiss wiring from context in its
/// presentation builder, so this is the fieldless published contract the frame's footer
/// region names as its `Render::Model`.
#[derive(Clone, PartialEq, Default)]
pub struct HelpFooterHostModel;

impl From<&HelpFooterHostView> for HelpFooterHostModel {
    fn from(_view: &HelpFooterHostView) -> Self {
        Self
    }
}

impl ddd::Model for HelpFooterHostModel {
    type View = HelpFooterHostView;
}
