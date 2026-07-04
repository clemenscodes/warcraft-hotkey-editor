use crate::{classes, styling::TailwindClass, tw};

/// The two-pane layout (sidebar column + fluid detail), self-contained here rather
const BASE: &[TailwindClass] = tw![
    "grid",
    "grid-cols-[34rem_minmax(0,1fr)]",
    "gap-10",
    "items-stretch",
    "mt-6",
    "flex-[1_1_0]",
    "min-h-0",
];
const MOBILE: &[TailwindClass] = tw![
    "mobile:grid-cols-[1fr]",
    "mobile:flex-none",
    "mobile:min-h-[auto]",
];
const TABLET: &[TailwindClass] = tw!["tablet:grid-cols-[18rem_minmax(0,1fr)]"];
const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw!["qhd:grid-cols-[46rem_minmax(0,1fr)]"];
const UHD: &[TailwindClass] = tw!["uhd:grid-cols-[62rem_minmax(0,1fr)]"];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
