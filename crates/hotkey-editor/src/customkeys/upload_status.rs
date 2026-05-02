#[derive(Clone, PartialEq)]
pub(crate) enum UploadStatus {
    Idle,
    Loading,
    Loaded {
        binding_count: usize,
        command_count: usize,
    },
    Error(String),
}
