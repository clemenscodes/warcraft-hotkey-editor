use dioxus::prelude::*;
use std::time::Duration;

/// The default lifetime of a non-permanent toast.
const DEFAULT_TOAST_DURATION: Duration = Duration::from_secs(5);

/// The visual kind of a toast. A consumer's visuals decide how each kind looks;
/// this enum can double as the `states!` overlay enum for a toast card/icon.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

impl ToastType {
    /// The `data-type` attribute value carried by the rendered toast.
    pub fn data_type(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
        }
    }
}

/// The payload handed to the provider's add callback. A named struct in place of
/// a tuple callback argument.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AddToastRequest {
    title: String,
    description: Option<String>,
    toast_type: ToastType,
    duration: Option<Duration>,
    permanent: bool,
}

/// One live toast in the provider's queue, with its final duration resolved.
#[derive(Clone, Debug, PartialEq)]
pub struct ToastRecord {
    id: usize,
    title: String,
    description: Option<String>,
    toast_type: ToastType,
    duration: Option<Duration>,
    permanent: bool,
}

impl ToastRecord {
    /// Build a record from an add request, materializing the default duration for
    /// a non-permanent toast that did not request one.
    pub(crate) fn new(id: usize, request: AddToastRequest) -> Self {
        let permanent = request.permanent;
        let duration = if permanent {
            None
        } else {
            request.duration.or(Some(DEFAULT_TOAST_DURATION))
        };
        Self {
            id,
            title: request.title,
            description: request.description,
            toast_type: request.toast_type,
            duration,
            permanent,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn toast_type(&self) -> ToastType {
        self.toast_type
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }

    pub fn permanent(&self) -> bool {
        self.permanent
    }
}

/// Options for a dispatched toast: an optional description, an optional custom
/// duration, and whether the toast is permanent (never auto-dismissed).
#[derive(Clone, Default)]
pub struct ToastOptions {
    description: Option<String>,
    duration: Option<Duration>,
    permanent: bool,
}

impl ToastOptions {
    /// A new, empty set of options: no description, default duration, not permanent.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the description shown under the title.
    pub fn description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Set a custom lifetime for the toast.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Mark the toast permanent, so it is never auto-dismissed.
    pub fn permanent(mut self, permanent: bool) -> Self {
        self.permanent = permanent;
        self
    }
}

/// The dispatch handle exposed by [`use_toast`]/[`consume_toast`]. A copyable
/// wrapper over the provider's add callback, provided as context by the
/// provider hook.
#[derive(Clone, Copy)]
pub struct Toasts {
    add_toast: Callback<AddToastRequest>,
}

impl Toasts {
    /// Construct the handle from the provider's add callback.
    pub(crate) fn new(add_toast: Callback<AddToastRequest>) -> Self {
        Self { add_toast }
    }

    /// Dispatch a toast of the given type with the given title and options.
    pub fn show(&self, title: String, toast_type: ToastType, options: ToastOptions) {
        let permanent = options.permanent;
        let duration = if permanent { None } else { options.duration };
        let request = AddToastRequest {
            title,
            description: options.description,
            toast_type,
            duration,
            permanent,
        };
        self.add_toast.call(request);
    }

    /// Dispatch a success toast.
    pub fn success(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Success, options);
    }

    /// Dispatch an error toast.
    pub fn error(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Error, options);
    }

    /// Dispatch a warning toast.
    pub fn warning(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Warning, options);
    }

    /// Dispatch an info toast.
    pub fn info(&self, title: String, options: ToastOptions) {
        self.show(title, ToastType::Info, options);
    }
}

/// Access the toast dispatch handle from the nearest provider. Call from a
/// component body (it is a hook).
pub fn use_toast() -> Toasts {
    use_hook(consume_toast)
}

/// Access the toast dispatch handle outside of hook position (from a `From` impl
/// or other non-hook code running under a provider).
pub fn consume_toast() -> Toasts {
    consume_context::<Toasts>()
}
