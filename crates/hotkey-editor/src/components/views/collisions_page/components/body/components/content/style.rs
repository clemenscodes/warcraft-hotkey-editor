use crate::classes;

/// The two-pane layout (sidebar column + fluid detail), self-contained here rather
const BASE: &[&str] = &[
    "grid",
    "grid-cols-[34rem_minmax(0,1fr)]",
    "gap-10",
    "items-stretch",
    "mt-6",
    "flex-[1_1_0]",
    "min-h-0",
];
const MOBILE: &[&str] = &[
    "mobile:grid-cols-[1fr]",
    "mobile:flex-none",
    "mobile:min-h-[auto]",
];
const TABLET: &[&str] = &["tablet:grid-cols-[18rem_minmax(0,1fr)]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &["qhd:grid-cols-[46rem_minmax(0,1fr)]"];
const UHD: &[&str] = &["uhd:grid-cols-[62rem_minmax(0,1fr)]"];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
