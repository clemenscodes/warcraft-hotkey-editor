#[derive(Clone, PartialEq)]
pub struct ToastIconView {
    pub icon: &'static str,
}

impl ddd::View for ToastIconView {}
