/// An opaque, component-private class list. See the crate docs: it has no
/// `Display`, no `Debug`, and no public accessor, so a `class:` attribute can
/// take it but nothing can append to it or read it out. `Clone`/`Copy` are the
/// only derives it qualifies for without breaking that opacity.
#[derive(Clone, Copy)]
pub struct ClassList {
    #[cfg_attr(not(feature = "dioxus"), allow(dead_code))]
    class: &'static str,
}

impl ClassList {
    /// Wrap an already-assembled class string. Only the generated `classes!`
    /// and `states!` macros call this.
    pub const fn new(class: &'static str) -> Self {
        Self { class }
    }

    /// Bridge for handing this class to a third-party component whose `class`
    /// prop is typed `String` rather than an attribute value (e.g. the
    /// `dioxus_primitives` `DialogContent`). Never called from a component body
    /// — bodies always use `class: CLASS` — so the no-interpolation guarantee
    /// still holds everywhere it matters.
    #[cfg(feature = "dioxus")]
    pub fn to_library_class(self) -> String {
        self.class.to_string()
    }
}

#[cfg(feature = "dioxus")]
impl dioxus::core::IntoAttributeValue for ClassList {
    fn into_value(self) -> dioxus::core::AttributeValue {
        dioxus::core::AttributeValue::Text(self.class.to_string())
    }
}
