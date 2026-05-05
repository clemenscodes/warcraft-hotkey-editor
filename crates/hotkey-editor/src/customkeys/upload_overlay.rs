use warcraft_keybinds::CustomKeysFile;

pub(crate) struct UploadOverlay;

impl UploadOverlay {
    pub(crate) fn apply(target_file: &mut CustomKeysFile, uploaded_file: &CustomKeysFile) {
        warcraft_keybinds::overlay::apply_overlay(target_file, uploaded_file);
    }
}
