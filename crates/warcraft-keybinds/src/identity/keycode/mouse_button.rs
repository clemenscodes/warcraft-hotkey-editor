use ddd::DomainLayer;
use ddd::Layered;
use ddd::ValueObject;

/// A mouse side button Warcraft III can bind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MouseButton {
    Back,
    Forward,
}

impl Layered for MouseButton {
    type Layer = DomainLayer;
}

impl ValueObject for MouseButton {}
