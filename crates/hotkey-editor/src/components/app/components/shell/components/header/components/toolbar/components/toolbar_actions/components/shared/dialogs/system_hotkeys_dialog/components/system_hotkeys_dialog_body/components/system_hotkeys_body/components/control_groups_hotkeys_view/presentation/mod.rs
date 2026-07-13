use warcraft_api::SystemHotkeysCategory;

/// The control-groups editor's shaped content: the intro caption the domain supplies for
/// the Control Groups category.
pub(super) struct ControlGroupsHotkeysViewModel {
    pub(super) caption: &'static str,
}

/// Sources the Control Groups category's intro caption from the domain, so the renderer
/// never hardcodes the copy.
pub(super) fn use_control_groups_hotkeys_view() -> ControlGroupsHotkeysViewModel {
    let caption = SystemHotkeysCategory::ControlGroups
        .caption()
        .unwrap_or_default();
    ControlGroupsHotkeysViewModel { caption }
}
