use crate::{classes, styling::TailwindClass, tw};

// The body of a position-picker dialog: a centered column holding the explainer and
// the grid. Shared by the off-state and upgraded-form pickers.
const BASE: &[TailwindClass] = tw![
    "flex",
    "flex-col",
    "items-center",
    "gap-6",
    "p-[2rem_2.5rem_2.5rem]",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
