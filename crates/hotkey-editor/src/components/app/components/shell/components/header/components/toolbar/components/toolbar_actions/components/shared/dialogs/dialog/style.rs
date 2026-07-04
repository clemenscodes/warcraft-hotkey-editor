use crate::styling::ClassList;
use crate::{classes, styling::TailwindClass, tw};

const BASE: &[TailwindClass] = tw![
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
    "bg-[linear-gradient(135deg,rgba(12,25,50,0.98)_0%,rgba(6,12,28,0.98)_100%)]",
    "shadow-[0_0_40px_rgba(255,206,99,0.25),0_8px_32px_rgba(0,0,0,0.6)]",
];

const MOBILE: &[TailwindClass] = tw![
    "mobile:w-screen",
    "mobile:h-dvh",
    "mobile:max-w-screen",
    "mobile:max-h-dvh",
    "mobile:rounded-none",
    "mobile:border-x-0",
];

const TABLET: &[TailwindClass] = tw![
    "tablet:w-[90vw]",
    "tablet:h-[90vh]",
    "tablet:max-w-[90vw]",
    "tablet:max-h-[90vh]",
];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

/// The backdrop (the `DialogRoot`): dims the page and centres the box. A fixed,
/// near-non-responsive concern, so it is a plain class list on the library
/// element rather than a second banded identity.
pub(super) const OVERLAY: ClassList = ClassList::new(
    "fixed inset-0 z-[1000] flex items-center justify-center p-8 bg-black/70 mobile:p-0",
);
