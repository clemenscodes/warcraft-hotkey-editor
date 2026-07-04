use dioxus::core::{AttributeValue, IntoAttributeValue};

/// An opaque, component-private class list. See the module docs: it has no
/// `Display` and no public accessor, so the `class:` attribute can take it but
/// nothing can append to it or read it out.
#[derive(Clone, Copy)]
pub struct ClassList(&'static str);

impl ClassList {
    /// Wrap an already-assembled class string. Only `classes!` calls this.
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Bridge for handing this class to a third-party component whose `class`
    /// prop is typed `String` rather than an attribute value (the
    /// `dioxus_primitives` `DialogContent` is the only such case). Crate-internal
    /// and never called from a component body — bodies always use `class: CLASS`,
    /// so the no-interpolation guarantee still holds everywhere it matters.
    pub(crate) fn to_library_class(self) -> String {
        self.0.to_string()
    }
}

impl IntoAttributeValue for ClassList {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.0.to_string())
    }
}
