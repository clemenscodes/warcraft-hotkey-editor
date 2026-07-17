use super::view::SearchScopePickerView;

#[derive(Clone, PartialEq, Default)]
pub struct SearchScopePickerModel;

impl From<&SearchScopePickerView> for SearchScopePickerModel {
    fn from(_view: &SearchScopePickerView) -> Self {
        Self
    }
}

impl ddd::Model for SearchScopePickerModel {
    type View = SearchScopePickerView;
}
