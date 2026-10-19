use warcraft_api::SystemHotkeysCategory;

pub(super) struct ControlGroupsHotkeysViewModel {
    pub(super) caption: &'static str,
}

pub(super) fn use_control_groups_hotkeys_view() -> ControlGroupsHotkeysViewModel {
    let caption = SystemHotkeysCategory::ControlGroups
        .caption()
        .unwrap_or_default();
    ControlGroupsHotkeysViewModel { caption }
}
