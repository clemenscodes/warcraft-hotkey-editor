use super::props::SystemHotkeysListViewProps;
use dioxus::prelude::*;
use warcraft_keybinds::SystemBindingMap;

/// Builds the binding map every row reads to flag conflicts.
pub(super) fn use_system_hotkeys_list_view(
    props: &SystemHotkeysListViewProps,
) -> Memo<SystemBindingMap> {
    let loaded_keys = props.loaded_keys;
    use_memo(move || {
        let guard = loaded_keys.read();
        SystemBindingMap::build(guard.as_ref())
    })
}
