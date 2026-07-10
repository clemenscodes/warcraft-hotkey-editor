use super::components::system_hotkeys_breadcrumbs_menu::SystemHotkeysBreadcrumbsMenuProps;
use super::components::system_hotkeys_breadcrumbs_trigger::SystemHotkeysBreadcrumbsTriggerProps;
use super::hooks::SystemHotkeysBreadcrumbsModel;

impl From<&SystemHotkeysBreadcrumbsModel> for SystemHotkeysBreadcrumbsTriggerProps {
    fn from(model: &SystemHotkeysBreadcrumbsModel) -> Self {
        let label = model.trigger_label.clone();
        let is_open = model.is_open;
        let on_toggle = model.on_toggle;
        Self {
            label,
            is_open,
            on_toggle,
        }
    }
}

impl From<&SystemHotkeysBreadcrumbsModel> for SystemHotkeysBreadcrumbsMenuProps {
    fn from(model: &SystemHotkeysBreadcrumbsModel) -> Self {
        let active_category = model.active_category;
        let picker_open = model.open;
        let is_open = model.is_open;
        Self {
            active_category,
            picker_open,
            is_open,
        }
    }
}
