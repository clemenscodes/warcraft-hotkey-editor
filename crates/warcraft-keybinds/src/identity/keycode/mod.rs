//! A precise keyboard key. Every value of [`KeyCode`] is a key Warcraft III
//! actually accepts in `CustomKeys.txt`; invalid codes cannot be represented.
//! The only way in from a raw number is [`TryFrom<u32>`], which rejects anything
//! that is not a real key, so `999` or `255` never become a `KeyCode`.
//!
//! Each key category is its own [`ddd::ValueObject`] in its own file; `KeyCode`
//! is the umbrella that unifies them and owns the `u32` round-trip.

pub mod digit;
pub mod function_key;
pub mod key_code;
pub mod letter;
pub mod mouse_button;
pub mod not_a_letter;
pub mod numpad_key;
pub mod out_of_range;
pub mod punctuation;

pub use digit::Digit;
pub use function_key::FunctionKey;
pub use key_code::KeyCode;
pub use letter::Letter;
pub use mouse_button::MouseButton;
pub use not_a_letter::NotALetter;
pub use numpad_key::NumpadKey;
pub use out_of_range::KeyCodeOutOfRange;
pub use punctuation::Punctuation;
