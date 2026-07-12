/// The published `View` contract mirroring [`SystemHotkeysSectionIntroModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysSectionIntroView {
    pub text: String,
}

impl ddd::View for SystemHotkeysSectionIntroView {}
