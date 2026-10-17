#[derive(Clone, PartialEq, Default)]
pub struct SearchScopePickerView;

impl ddd::Layered for SearchScopePickerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SearchScopePickerView {}
