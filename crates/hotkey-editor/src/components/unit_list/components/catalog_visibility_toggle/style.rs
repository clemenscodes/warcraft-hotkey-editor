use crate::classes;

// The No-abilities / All-variants visibility toggle group. A side-by-side pair; the
// child buttons are tall on the sidebar and shorter on mobile.
const BASE: &[&str] = &[
    "flex",
    "flex-row",
    "gap-2",
    "mb-2",
    "[&>button]:min-h-[6.7rem]!",
];

const MOBILE: &[&str] = &["mobile:[&>button]:min-h-[3.5rem]!"];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
