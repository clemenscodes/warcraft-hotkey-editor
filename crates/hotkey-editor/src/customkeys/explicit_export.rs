use warcraft_keybinds::CustomKeysFile;

use crate::customkeys::baseline::baseline_content;

pub(crate) struct ExplicitExport;

impl ExplicitExport {
    pub(crate) fn serialize(loaded_file: &CustomKeysFile) -> String {
        warcraft_keybinds::export::serialize(loaded_file, baseline_content())
    }
}
