use warcraft_keybinds::CustomKeysFile;

pub(crate) struct UploadOverlay;

impl UploadOverlay {
    pub(crate) fn apply(target_file: &mut CustomKeysFile, uploaded_file: &CustomKeysFile) {
        target_file.overlay(uploaded_file);
    }
}
