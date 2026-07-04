use crate::{classes, styling::TailwindClass, tw};

// The Unit/Ability search-field toggle group. A stacked pair that becomes a side-by-
// side row on small screens; the child buttons are tall on the sidebar and shorter
// on mobile.
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "gap-2",
    "mb-2",
    "[&>button]:min-h-[6.7rem]!",
];

const MOBILE: &[TailwindClass] = tw!["mobile:flex-row", "mobile:[&>button]:min-h-[3.5rem]!"];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
