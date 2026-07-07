use tw_macro::ClassList;
use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "flex-col",
        "w-[80vw]",
        "h-[80vh]",
        "p-0",
        "gap-0",
        "overflow-hidden",
        "rounded-xl",
        "border",
        "border-warcraft-gold",
        "bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-bg-mid)_98%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-bg-base)_98%,transparent)_100%)]",
        "shadow-ambient-gold",
    ],
    mobile: tw![
        "mobile:w-screen",
        "mobile:h-dvh",
        "mobile:max-w-screen",
        "mobile:max-h-dvh",
        "mobile:rounded-none",
        "mobile:border-x-0",
    ],
    tablet: tw![
        "tablet:w-[90vw]",
        "tablet:h-[90vh]",
        "tablet:max-w-[90vw]",
        "tablet:max-h-[90vh]",
    ],
}

/// The backdrop (the `DialogRoot`): dims the page and centres the box. A fixed,
/// near-non-responsive concern, so it is a plain class list on the library
/// element rather than a second banded identity.
pub(super) const OVERLAY: ClassList = ClassList::new(
    "fixed inset-0 z-[1000] flex items-center justify-center p-8 bg-black/70 mobile:p-0",
);
