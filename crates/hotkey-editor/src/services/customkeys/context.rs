use dioxus::prelude::*;

use crate::services::customkeys::service::CustomKeysService;
use crate::services::customkeys::upload_status::UploadStatus;

/// Access the [`CustomKeysService`] provided at the app root. Call from a
/// component or hook body (it is a hook). The service is a global, crate-wide
/// concept — no single component owns it — so this accessor lives beside the
/// type in `services/`, not colocated with any component.
pub(crate) fn use_custom_keys_service() -> CustomKeysService {
    use_context::<CustomKeysService>()
}

/// Access the app-wide [`UploadStatus`] signal provided at the app root. Call from
/// a component or hook body (it is a hook). Upload progress is a global, crate-wide
/// concept owned by the customkeys domain, so this accessor lives beside the type
/// in `services/customkeys/`, not colocated with any component.
pub(crate) fn use_upload_status() -> Signal<UploadStatus> {
    use_context::<Signal<UploadStatus>>()
}
