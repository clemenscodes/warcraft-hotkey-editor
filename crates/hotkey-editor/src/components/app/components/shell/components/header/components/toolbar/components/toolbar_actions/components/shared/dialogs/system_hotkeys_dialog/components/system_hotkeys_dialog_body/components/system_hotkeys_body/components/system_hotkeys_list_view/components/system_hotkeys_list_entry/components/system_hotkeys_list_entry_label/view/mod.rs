/// The published `View` contract mirroring [`SystemHotkeysListEntryLabelModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct SystemHotkeysListEntryLabelView {
    pub text: String,
}

impl ddd::View for SystemHotkeysListEntryLabelView {}
