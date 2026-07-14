use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::services::customkeys::service::CustomKeysService;
use crate::services::customkeys::upload_status::UploadStatus;

pub(crate) fn use_loaded_keys() -> Signal<Option<CustomKeys>> {
    use_context()
}

pub(crate) fn use_custom_keys_service() -> CustomKeysService {
    use_context()
}

pub(crate) fn use_upload_status() -> Signal<UploadStatus> {
    use_context()
}
