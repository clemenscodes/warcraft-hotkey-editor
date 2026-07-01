use crate::classes;

// The Unit/Ability search-field toggle group. A stacked pair that becomes a side-by-
// side row on small screens; the child buttons are tall on the sidebar and shorter
// on mobile.
const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "gap-2",
    "mb-2",
    "[&>button]:min-h-[6.7rem]!",
];

const MOBILE: &[&str] = &["mobile:flex-row", "mobile:[&>button]:min-h-[3.5rem]!"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
