use crate::{classes, styling::TailwindClass, tw};

// The No-abilities / All-variants visibility toggle group. A side-by-side pair; the
// child buttons are tall on the sidebar and shorter on mobile.
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-row",
    "gap-2",
    "mb-2",
    "[&>button]:min-h-[6.7rem]!",
];

const MOBILE: &[TailwindClass] = tw!["mobile:[&>button]:min-h-[3.5rem]!"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
