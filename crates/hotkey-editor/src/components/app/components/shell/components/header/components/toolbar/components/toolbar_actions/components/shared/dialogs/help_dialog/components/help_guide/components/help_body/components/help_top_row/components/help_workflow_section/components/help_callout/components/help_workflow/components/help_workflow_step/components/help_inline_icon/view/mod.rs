#[derive(Clone, PartialEq)]
pub struct HelpInlineIconView {
    pub icon: &'static str,
}

impl ddd::View for HelpInlineIconView {}
