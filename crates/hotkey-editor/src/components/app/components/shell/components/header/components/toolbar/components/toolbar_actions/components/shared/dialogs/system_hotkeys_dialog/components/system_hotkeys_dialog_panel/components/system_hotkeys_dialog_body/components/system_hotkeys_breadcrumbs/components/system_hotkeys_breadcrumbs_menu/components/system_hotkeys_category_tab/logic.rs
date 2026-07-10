use super::components::active_category_tab::ActiveCategoryTabProps;
use super::components::inactive_category_tab::InactiveCategoryTabProps;
use super::components::popover_active_category_tab::PopoverActiveCategoryTabProps;
use super::components::popover_inactive_category_tab::PopoverInactiveCategoryTabProps;
use super::hooks::SystemHotkeysCategoryTabModel;

impl From<&SystemHotkeysCategoryTabModel> for ActiveCategoryTabProps {
    fn from(model: &SystemHotkeysCategoryTabModel) -> Self {
        let label = model.label.clone();
        let on_click = model.on_click;
        Self { label, on_click }
    }
}

impl From<&SystemHotkeysCategoryTabModel> for InactiveCategoryTabProps {
    fn from(model: &SystemHotkeysCategoryTabModel) -> Self {
        let label = model.label.clone();
        let on_click = model.on_click;
        Self { label, on_click }
    }
}

impl From<&SystemHotkeysCategoryTabModel> for PopoverActiveCategoryTabProps {
    fn from(model: &SystemHotkeysCategoryTabModel) -> Self {
        let label = model.label.clone();
        let on_click = model.on_click;
        Self { label, on_click }
    }
}

impl From<&SystemHotkeysCategoryTabModel> for PopoverInactiveCategoryTabProps {
    fn from(model: &SystemHotkeysCategoryTabModel) -> Self {
        let label = model.label.clone();
        let on_click = model.on_click;
        Self { label, on_click }
    }
}
