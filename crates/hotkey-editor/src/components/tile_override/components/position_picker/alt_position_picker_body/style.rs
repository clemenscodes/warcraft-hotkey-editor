use crate::classes;

// The body of a position-picker dialog: a centered column holding the explainer and
// the grid. Shared by the off-state and upgraded-form pickers.
const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-center",
    "gap-6",
    "p-[2rem_2.5rem_2.5rem]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
