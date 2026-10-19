#[derive(Clone, PartialEq)]
pub enum UploadStatus {
    Idle,
    Loading,
    Loaded {
        binding_count: usize,
        command_count: usize,
    },
    Error(String),
}
