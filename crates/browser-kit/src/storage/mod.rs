//! Browser `localStorage` primitives: a keyed get/set wrapper and string codecs.

mod compressed_text;
mod local_storage;

pub use compressed_text::CompressedText;
pub use local_storage::LocalStorage;
